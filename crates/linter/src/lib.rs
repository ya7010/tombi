mod options;
use diagnostics::Diagnostic;
pub use options::Options;

pub fn lint(source: &str) -> Result<(), Vec<Diagnostic>> {
    lint_with_option(source, &Options::default())
}
pub fn lint_with_option(source: &str, _options: &crate::Options) -> Result<(), Vec<Diagnostic>> {
    let p = parser::parse(source);
    let errors = p.errors();

    if errors.is_empty() {
        return Ok(());
    }

    Err(errors
        .into_iter()
        .map(|error| Diagnostic::new_error(error.message(), error.range()))
        .collect())
}
