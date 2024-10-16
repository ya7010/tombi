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
pub struct SyntaxError(String, text_size::TextRange);

impl SyntaxError {
    pub fn new(message: impl Into<String>, range: impl IntoRange) -> Self {
        Self(message.into(), range.into_range())
    }

    pub fn range(&self) -> text_size::TextRange {
        self.1
    }

    pub fn with_range(mut self, range: text_size::TextRange) -> Self {
        self.1 = range;
        self
    }
}
