pub use config::LintOptions;
use config::TomlVersion;
use diagnostic::Diagnostic;

pub fn lint(source: &str) -> Result<(), Vec<Diagnostic>> {
    lint_with(source, TomlVersion::default(), &LintOptions::default())
}

pub fn lint_with(
    source: &str,
    toml_version: TomlVersion,
    _options: &crate::LintOptions,
) -> Result<(), Vec<Diagnostic>> {
    let p = parser::parse(source, toml_version);
    let errors = p.errors();

    if errors.is_empty() {
        return Ok(());
    }

    Err(errors
        .into_iter()
        .map(|error| Diagnostic::new_error(error.message(), error.range()))
        .collect())
}
