#[derive(thiserror::Error, Default, Debug, Clone, PartialEq)]
pub enum Error {
    #[default]
    #[error("Invalid token")]
    InvalidToken,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError(String, text_size::TextRange);

impl SyntaxError {
    pub fn new(message: impl Into<String>, range: text_size::TextRange) -> Self {
        Self(message.into(), range)
    }

    pub fn new_at_offset(message: impl Into<String>, offset: text_size::TextSize) -> Self {
        Self(message.into(), text_size::TextRange::empty(offset))
    }

    pub fn range(&self) -> text_size::TextRange {
        self.1
    }

    pub fn with_range(mut self, range: text_size::TextRange) -> Self {
        self.1 = range;
        self
    }
}
