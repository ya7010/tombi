mod error;
mod lint;
mod linter;
mod rule;
mod validate;
mod warning;

pub use config::LintOptions;
use diagnostic::Diagnostic;
pub use error::{Error, ErrorKind};
use lint::Lint;
pub use linter::Linter;
use rule::Rule;
pub use validate::Validate;
pub use warning::{Warning, WarningKind};
