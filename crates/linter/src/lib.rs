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
