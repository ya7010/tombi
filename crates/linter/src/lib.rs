mod error;
mod lint;
mod linter;
mod rule;
use error::ErrorKind;

use ast::AstNode;
pub use config::LintOptions;
use config::TomlVersion;
use diagnostic::Diagnostic;
use diagnostic::ToDiagnostics;
use lint::Lint;
use linter::Linter;
use rule::Rule;

pub fn lint(source: &str) -> Result<(), Vec<Diagnostic>> {
    lint_with(source, TomlVersion::default(), &LintOptions::default())
}

pub fn lint_with(
    source: &str,
    toml_version: TomlVersion,
    _options: &LintOptions,
) -> Result<(), Vec<Diagnostic>> {
    let p = parser::parse(source, toml_version);
    let mut errors = vec![];

    for err in p.errors() {
        err.to_diagnostics(&mut errors);
    }

    if errors.is_empty() {
        if let Some(root) = ast::Root::cast(p.into_syntax_node()) {
            let mut linter = Linter::new(toml_version, _options);
            root.lint(&mut linter);
            errors.extend(linter.into_diagnostics());

            if let Err(errs) = document_tree::Root::try_from(root) {
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
