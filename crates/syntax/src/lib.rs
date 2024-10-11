mod error;
mod syntax_kind;

pub use error::Error;
pub use syntax_kind::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TomlLanguage {}

impl rowan::Language for TomlLanguage {
    type Kind = crate::SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= crate::SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, crate::SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<crate::TomlLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<crate::TomlLanguage>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<TomlLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<TomlLanguage>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<TomlLanguage>;
