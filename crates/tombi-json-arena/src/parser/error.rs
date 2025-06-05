#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    #[error("Unexpected token")] // 詳細はParserError側で持つ
    UnexpectedToken,
    #[error("Expected token")] // 詳細はParserError側で持つ
    ExpectedToken,
}

#[derive(Debug, thiserror::Error)]
#[error("{kind} at {range:?}")]
pub struct Error {
    pub kind: ErrorKind,
    pub range: tombi_text::Range,
}
