#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    span: text::Span,
    error: syntax::Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidOffsetDatetime,
    InvalidLocalDatetime,
    InvalidLocalDate,
    InvalidLocalTime,
}

impl Error {
    pub fn new(kind: ErrorKind, span: text::Span, error: syntax::Error) -> Self {
        Self { kind, span, error }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn msg(&self) -> &str {
        self.error.as_str()
    }
}
