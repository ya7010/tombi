#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TomlLang;

impl rowan::Language for TomlLang {
    type Kind = crate::Token;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= crate::Token::ROOT as u16);
        unsafe { std::mem::transmute::<u16, crate::Token>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
