use std::borrow::Cow;

use crate::lint::Lint;
use ast::AstNode;
use config::TomlVersion;
use diagnostic::Diagnostic;
use diagnostic::SetDiagnostics;
use document_tree::TryIntoDocumentTree;

pub struct Linter<'a> {
    toml_version: TomlVersion,
    options: Cow<'a, crate::LintOptions>,
    #[allow(dead_code)]
    schema_store: &'a schema_store::SchemaStore,
    diagnostics: Vec<crate::Diagnostic>,
}

impl<'a> Linter<'a> {
    #[inline]
    pub fn new(
        toml_version: TomlVersion,
        options: &'a crate::LintOptions,
        schema_store: &'a schema_store::SchemaStore,
    ) -> Self {
        Self {
            toml_version,
            options: Cow::Borrowed(options),
            schema_store,
            diagnostics: Vec::new(),
        }
    }

    pub async fn lint(mut self, source: &str) -> Result<(), Vec<Diagnostic>> {
        let toml_version = self.toml_version;
        let p = parser::parse(source, toml_version);
        let mut errors = vec![];

        for err in p.errors() {
            err.set_diagnostic(&mut errors);
        }

        if errors.is_empty() {
            let Some(root) = ast::Root::cast(p.into_syntax_node()) else {
                unreachable!("Root node is always present");
            };

            root.lint(&mut self);
            errors.extend(self.into_diagnostics());

            if let Err(errs) = root.try_into_document_tree(toml_version) {
                for err in errs {
                    err.set_diagnostic(&mut errors);
                }
            }
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
    pub(crate) fn into_diagnostics(self) -> Vec<crate::Diagnostic> {
        self.diagnostics
    }

    #[inline]
    pub(crate) fn add_diagnostic(&mut self, diagnostic: crate::Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}
