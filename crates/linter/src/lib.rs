mod error;
mod lint;
mod linter;
mod rule;

pub use config::LintOptions;
pub use linter::Linter;

use config::TomlVersion;
use diagnostic::Diagnostic;
use error::ErrorKind;
use lint::Lint;
use rule::Rule;

pub fn lint(source: &str) -> Result<(), Vec<Diagnostic>> {
    Linter::new(TomlVersion::default(), &LintOptions::default()).lint(source)
}
