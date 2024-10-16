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

pub trait IntoRange {
    fn into_range(self) -> text_size::TextRange;
}

impl IntoRange for text_size::TextRange {
    fn into_range(self) -> text_size::TextRange {
        self
    }
}

impl IntoRange for std::ops::Range<u32> {
    fn into_range(self) -> text_size::TextRange {
        text_size::TextRange::new(self.start.into(), self.end.into())
    }
}

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
