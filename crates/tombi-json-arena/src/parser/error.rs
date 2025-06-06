#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    #[error("Unexpected token")]
    UnexpectedToken,
    #[error("Expected token")]
    ExpectedToken,
    #[error("Expected value")]
    ExpectedValue,
}

#[derive(Debug, thiserror::Error)]
#[error("{kind} at {range:?}")]
pub struct Error {
    pub kind: ErrorKind,
    pub range: tombi_text::Range,
}
