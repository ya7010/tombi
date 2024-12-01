#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ErrorKind {
    #[error("expected key")]
    ExpectedKey,

    #[error("expected value")]
    ExpectedValue,

    #[error("unknown token")]
    UnknownToken,

    #[error("unknown line")]
    UnknownLine,

    #[error("expected '='")]
    ExpectedEqual,

    #[error("expected ','")]
    ExpectedComma,

    #[error("expected ']'")]
    ExpectedBracketEnd,

    #[error("expected ']]'")]
    ExpectedDoubleBracketEnd,

    #[error("expected '}}'")]
    ExpectedBraceEnd,

    #[error("expected '\\n' or comment")]
    ExpectedLineBreakOrComment,

    #[error("Invalid key")]
    InvalidKey,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    span: text::Span,
}

impl Error {
    pub fn new(kind: ErrorKind, span: text::Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn span(&self) -> text::Span {
        self.span
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}
