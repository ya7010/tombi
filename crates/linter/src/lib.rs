mod options;
use diagnostic::Diagnostic;
pub use options::Options;
use syntax::TomlVersion;

pub fn lint(source: &str) -> Result<(), Vec<Diagnostic>> {
    lint_with(source, TomlVersion::default(), &Options::default())
}
pub fn lint_with(
    source: &str,
    toml_version: TomlVersion,
    _options: &crate::Options,
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
