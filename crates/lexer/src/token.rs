use syntax::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    kind: SyntaxKind,
    span: text::Span,
}

impl Token {
    pub fn new(kind: SyntaxKind, span: text::Span) -> Self {
        Self { kind, span }
    }

    pub const fn eof() -> Self {
        Self {
            kind: SyntaxKind::EOF,
            span: text::Span::new(text::Offset::new(0), text::Offset::new(0)),
        }
    }

    #[inline]
    pub fn is_eof(&self) -> bool {
        self.kind == SyntaxKind::EOF
    }

    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    #[inline]
    pub fn span(&self) -> text::Span {
        self.span
    }
}
