use std::borrow::Cow;

use crate::lint::Lint;
use ast::AstNode;
use config::TomlVersion;
use diagnostic::Diagnostic;
use diagnostic::SetDiagnostics;
use document_tree::IntoDocumentTreeResult;
use itertools::Either;
use schema_store::DocumentSchema;
use url::Url;

pub struct Linter<'a> {
    toml_version: TomlVersion,
    options: Cow<'a, crate::LintOptions>,
    #[allow(dead_code)]
    document_schema: Option<DocumentSchema>,
    #[allow(dead_code)]
    schema_store: &'a schema_store::SchemaStore,
    pub(crate) diagnostics: Vec<crate::Diagnostic>,
}

impl<'a> Linter<'a> {
    #[inline]
    pub async fn try_new(
        toml_version: TomlVersion,
        options: &'a crate::LintOptions,
        schema_url_or_path: Option<Either<&'a Url, &'a std::path::Path>>,
        schema_store: &'a schema_store::SchemaStore,
    ) -> Result<Self, schema_store::Error> {
        let document_schema = match schema_url_or_path {
            Some(schema_url_or_path) => {
                Some(schema_store.try_get_schema(schema_url_or_path).await?)
            }
            None => None,
        }
        .flatten();

        let toml_version = document_schema
            .as_ref()
            .and_then(|s| s.toml_version())
            .unwrap_or(toml_version);

        Ok(Self {
            toml_version,
            options: Cow::Borrowed(options),
            document_schema,
            schema_store,
            diagnostics: Vec::new(),
        })
    }

    pub async fn lint(mut self, source: &str) -> Result<(), Vec<Diagnostic>> {
        let p = parser::parse(source, self.toml_version);
        let mut errors = vec![];

        for err in p.errors() {
            err.set_diagnostic(&mut errors);
        }

        if errors.is_empty() {
            let Some(root) = ast::Root::cast(p.into_syntax_node()) else {
                unreachable!("Root node is always present");
            };

            root.lint(&mut self);

            let (document_tree, errs) = root.into_document_tree_result(self.toml_version).into();

            for err in errs {
                err.set_diagnostic(&mut errors);
            }

            if let Some(document_schema) = self.document_schema {
                if let Err(errs) =
                    crate::validation::validate(document_tree, self.toml_version, &document_schema)
                {
                    for err in errs {
                        err.set_diagnostic(&mut errors);
                    }
                }
            }

            errors.extend(self.diagnostics);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
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
