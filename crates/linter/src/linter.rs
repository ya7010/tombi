use std::borrow::Cow;

use crate::lint::Lint;
use ast::AstNode;
use config::TomlVersion;
use diagnostic::Diagnostic;
use diagnostic::ToDiagnostics;
use document_tree::TryIntoDocumentTree;

pub struct Linter<'a> {
    toml_version: TomlVersion,
    options: Cow<'a, crate::LintOptions>,
    diagnostics: Vec<crate::Diagnostic>,
}

impl<'a> Linter<'a> {
    #[inline]
    pub fn new(toml_version: TomlVersion, options: &'a crate::LintOptions) -> Self {
        Self {
            toml_version,
            options: Cow::Borrowed(options),
            diagnostics: Vec::new(),
        }
    }

    pub fn lint(self, source: &str) -> Result<(), Vec<Diagnostic>> {
        let p = parser::parse(source, self.toml_version);
        let mut errors = vec![];

        for err in p.errors() {
            err.to_diagnostics(&mut errors);
        }

        if errors.is_empty() {
            if let Some(root) = ast::Root::cast(p.into_syntax_node()) {
                let mut linter = Linter::new(self.toml_version, &self.options);
                root.lint(&mut linter);
                errors.extend(linter.into_diagnostics());

                if let Err(errs) = root.try_into_document_tree(self.toml_version) {
                    for err in errs {
                        err.to_diagnostics(&mut errors);
                    }
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
