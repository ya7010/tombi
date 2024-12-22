#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    span: text::Span,
    range: text::Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidKey,
    InvalidBasicString,
    InvalidLiteralString,
    InvalidMultilineBasicString,
    InvalidMultilineLiteralString,
    InvalidNumber,
    InvalidOffsetDateTime,
    InvalidLocalDateTime,
    InvalidLocalDate,
    InvalidLocalTime,
    InvalidLineBreak,
    InvalidToken,
}

impl Error {
    #[inline]
    pub fn new(kind: ErrorKind, (span, range): (text::Span, text::Range)) -> Self {
        Self { kind, span, range }
    }

    #[inline]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[inline]
    pub fn span(&self) -> text::Span {
        self.span
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }
}
