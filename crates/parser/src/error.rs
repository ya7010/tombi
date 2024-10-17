#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Error {
    #[error("expected key")]
    ExpectedKey,
    #[error("expected value")]
    ExpectedValue,
    #[error("unknown token")]
    UnknownToken,
    #[error("unknown line")]
    UnknownLine,
    #[error("expected '='")]
    ExpectedEquals,
    #[error("expected '{{'")]
    ExpectedBracketStart,
    #[error("expected '}}'")]
    ExpectedBracketEnd,
}

impl Into<String> for Error {
    fn into(self) -> String {
        self.to_string()
    }
}
