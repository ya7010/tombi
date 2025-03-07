use std::borrow::Cow;

use ast::AstNode;
use config::TomlVersion;
use diagnostic::{Diagnostic, SetDiagnostics};
use document_tree::IntoDocumentTreeAndErrors;
use itertools::Either;
use schema_store::SourceSchema;
use url::Url;

use crate::lint::Lint;

pub struct Linter<'a> {
    toml_version: TomlVersion,
    options: Cow<'a, crate::LintOptions>,
    source_schema: Option<SourceSchema>,
    schema_store: &'a schema_store::SchemaStore,
    pub(crate) diagnostics: Vec<crate::Diagnostic>,
}

impl<'a> Linter<'a> {
    pub async fn try_new(
        toml_version: TomlVersion,
        options: &'a crate::LintOptions,
        schema_url_or_path: Option<Either<&'a Url, &'a std::path::Path>>,
        schema_store: &'a schema_store::SchemaStore,
    ) -> Result<Self, schema_store::Error> {
        let source_schema = if let Some(schema_url_or_path) = schema_url_or_path {
            schema_store
                .try_get_source_schema(schema_url_or_path)
                .await?
        } else {
            None
        };

        let toml_version = source_schema
            .as_ref()
            .and_then(|source_schema| {
                source_schema
                    .root_schema
                    .as_ref()
                    .and_then(|document_schema| document_schema.toml_version())
            })
            .unwrap_or(toml_version);

        Ok(Self {
            toml_version,
            options: Cow::Borrowed(options),
            source_schema,
            schema_store,
            diagnostics: Vec::new(),
        })
    }

    pub async fn lint(mut self, source: &str) -> Result<(), Vec<Diagnostic>> {
        let p = parser::parse(source, self.toml_version);
        let mut diagnostics = vec![];

        for errors in p.errors() {
            errors.set_diagnostics(&mut diagnostics);
        }

        if diagnostics.is_empty() {
            let Some(root) = ast::Root::cast(p.into_syntax_node()) else {
                unreachable!("Root node is always present");
            };

            root.lint(&mut self);

            let (document_tree, errors) =
                root.into_document_tree_and_errors(self.toml_version).into();

            errors.set_diagnostics(&mut diagnostics);

            if let Some(source_schema) = &self.source_schema {
                let schema_context = schema_store::SchemaContext {
                    toml_version: self.toml_version,
                    root_schema: source_schema.root_schema.as_ref(),
                    sub_schema_url_map: Some(&source_schema.sub_schema_url_map),
                    store: self.schema_store,
                };
                if let Err(schema_diagnostics) =
                    crate::validation::validate(document_tree, source_schema, &schema_context).await
                {
                    diagnostics.extend(schema_diagnostics);
                }
            }

            diagnostics.extend(self.diagnostics);
        }

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn toml_version(&self) -> TomlVersion {
        self.toml_version
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn options(&self) -> &crate::LintOptions {
        &self.options
    }

    #[inline]
    pub(crate) fn add_diagnostic(&mut self, diagnostic: crate::Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}
