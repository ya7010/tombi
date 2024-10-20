use crate::IntoRange;

#[derive(thiserror::Error, Default, Debug, Clone, PartialEq)]
pub enum Error {
    #[default]
    #[error("Invalid token")]
    InvalidToken,
}

impl Error {
    pub fn as_str(&self) -> &str {
        match self {
            Self::InvalidToken => "Invalid token",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError {
    message: String,
    range: text_size::TextRange,
}

impl SyntaxError {
    pub fn new(message: impl Into<String>, range: impl IntoRange) -> Self {
        Self {
            message: message.into(),
            range: range.into_range(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn range(&self) -> text_size::TextRange {
        self.range
    }

    pub fn with_range(mut self, range: text_size::TextRange) -> Self {
        self.range = range;
        self
    }
}
