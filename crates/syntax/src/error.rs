#[derive(thiserror::Error, Default, Debug, Clone, PartialEq, Eq)]
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
    range: text::Range,
}

impl SyntaxError {
    pub fn new(message: impl Into<String>, range: text::Range) -> Self {
        Self {
            message: message.into(),
            range,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn position(&self) -> text::Position {
        self.range.start()
    }

    pub fn range(&self) -> text::Range {
        self.range
    }
}
