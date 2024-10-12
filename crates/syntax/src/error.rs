#[derive(thiserror::Error, Default, Debug, Clone, PartialEq)]
pub enum Error {
    #[default]
    #[error("Invalid token")]
    InvalidToken,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError(String, text_size::TextRange);
