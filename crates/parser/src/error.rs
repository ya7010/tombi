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
    #[error("expected ']'")]
    ExpectedBracketEnd,
    #[error("expected '}}'")]
    ExpectedBraceEnd,
    #[error("expected '}}}}'")]
    ExpectedDoubleBracetEnd,
    #[error("Invalid key")]
    InvalidKey,
}

impl Into<String> for Error {
    fn into(self) -> String {
        self.to_string()
    }
}
