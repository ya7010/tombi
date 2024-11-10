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
    pos: u32,
}

impl SyntaxError {
    pub fn new(message: impl Into<String>, pos: u32) -> Self {
        Self {
            message: message.into(),
            pos,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn pos(&self) -> u32 {
        self.pos
    }
}
