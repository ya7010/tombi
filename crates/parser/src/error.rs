#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Error {
    #[error("expected key")]
    ExpectedKey,
    #[error("expected value")]
    ExpectedValue,
    #[error("expected '='")]
    ExpectedEquals,
    #[error("unknown token")]
    UnknownToken,
}

impl Into<String> for Error {
    fn into(self) -> String {
        self.to_string()
    }
}
