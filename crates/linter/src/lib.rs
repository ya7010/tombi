mod error;
mod lint;
mod linter;
mod rule;

pub use config::LintOptions;
pub use linter::Linter;

use diagnostic::Diagnostic;
use error::ErrorKind;
use lint::Lint;
use rule::Rule;

pub fn lint(source: &str) -> Result<(), Vec<Diagnostic>> {
    let config = config::load();

    Linter::new(
        config.toml_version.unwrap_or_default(),
        &config.lint.unwrap_or_default(),
        &mut schema_store::SchemaStore::default(),
    )
    .lint(source)
}
