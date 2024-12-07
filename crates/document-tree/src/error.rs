#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("duplicate key: {key}")]
    DuplicateKey { key: String, range: text::Range },

    #[error("conflicting array.")]
    ConflictArray {
        range1: text::Range,
        range2: text::Range,
    },

    #[error("invalid integer: {error}")]
    ParseIntError {
        error: std::num::ParseIntError,
        range: text::Range,
    },

    #[error("invalid float: {error}")]
    ParseFloatError {
        error: std::num::ParseFloatError,
        range: text::Range,
    },

    #[error("invalid offset date time: {error}")]
    ParseOffsetDateTimeError {
        error: chrono::ParseError,
        range: text::Range,
    },

    #[error("invalid local date time: {error}")]
    ParseLocalDateTimeError {
        error: chrono::ParseError,
        range: text::Range,
    },

    #[error("invalid local date: {error}")]
    ParseLocalDateError {
        error: chrono::format::ParseError,
        range: text::Range,
    },

    #[error("invalid local time: {error}")]
    ParseLocalTimeError {
        error: chrono::format::ParseError,
        range: text::Range,
    },
}

#[cfg(feature = "diagnostic")]
impl diagnostic::ToDiagnostics for Error {
    fn to_diagnostics(&self, diagnostics: &mut Vec<diagnostic::Diagnostic>) {
        match self {
            Self::DuplicateKey { range, .. } => {
                diagnostics.push(diagnostic::Diagnostic::new_error(self.to_string(), *range));
            }
            Self::ConflictArray { range1, range2 } => {
                let diagnostic1 = diagnostic::Diagnostic::new_error(self.to_string(), *range1);
                if !diagnostics.contains(&diagnostic1) {
                    diagnostics.push(diagnostic1);
                }
                diagnostics.push(diagnostic::Diagnostic::new_error(self.to_string(), *range2));
            }
            Self::ParseIntError { range, .. } => {
                diagnostics.push(diagnostic::Diagnostic::new_error(self.to_string(), *range));
            }
            Self::ParseFloatError { range, .. } => {
                diagnostics.push(diagnostic::Diagnostic::new_error(self.to_string(), *range));
            }
            Self::ParseOffsetDateTimeError { range, .. } => {
                diagnostics.push(diagnostic::Diagnostic::new_error(self.to_string(), *range));
            }
            Self::ParseLocalDateTimeError { range, .. } => {
                diagnostics.push(diagnostic::Diagnostic::new_error(self.to_string(), *range));
            }
            Self::ParseLocalDateError { range, .. } => {
                diagnostics.push(diagnostic::Diagnostic::new_error(self.to_string(), *range));
            }
            Self::ParseLocalTimeError { range, .. } => {
                diagnostics.push(diagnostic::Diagnostic::new_error(self.to_string(), *range));
            }
        }
    }
}
