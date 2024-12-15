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
        error: crate::support::float::ParseFloatError,
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

impl Error {
    pub fn to_message(&self) -> String {
        self.to_string()
    }

    pub fn range(&self) -> text::Range {
        match self {
            Self::DuplicateKey { range, .. } => *range,
            Self::ConflictArray { range2, .. } => *range2,
            Self::ParseIntError { range, .. } => *range,
            Self::ParseFloatError { range, .. } => *range,
            Self::ParseOffsetDateTimeError { range, .. } => *range,
            Self::ParseLocalDateTimeError { range, .. } => *range,
            Self::ParseLocalDateError { range, .. } => *range,
            Self::ParseLocalTimeError { range, .. } => *range,
        }
    }
}

#[cfg(feature = "diagnostic")]
impl diagnostic::ToDiagnostics for Error {
    fn to_diagnostics(&self, diagnostics: &mut Vec<diagnostic::Diagnostic>) {
        match self {
            Self::ConflictArray { range1, range2 } => {
                let diagnostic1 = diagnostic::Diagnostic::new_error(self.to_message(), *range1);
                if !diagnostics.contains(&diagnostic1) {
                    diagnostics.push(diagnostic1);
                }
                diagnostics.push(diagnostic::Diagnostic::new_error(
                    self.to_message(),
                    *range2,
                ));
            }
            _ => {
                diagnostics.push(diagnostic::Diagnostic::new_error(
                    self.to_message(),
                    self.range(),
                ));
            }
        }
    }
}
