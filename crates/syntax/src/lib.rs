mod builder;
mod error;
mod generated;

pub use builder::SyntaxTreeBuilder;
pub use error::{Error, SyntaxError};
pub use generated::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TomlLanguage {}

impl rg_tree::Language for TomlLanguage {
    type Kind = crate::SyntaxKind;

    fn kind_from_raw(raw: rg_tree::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= crate::SyntaxKind::__LAST as u16);
        unsafe { std::mem::transmute::<u16, crate::SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rg_tree::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rg_tree::RedNode<TomlLanguage>;
pub type SyntaxToken = rg_tree::RedToken<TomlLanguage>;
pub type SyntaxElement = rg_tree::RedElement<TomlLanguage>;
pub type SyntaxNodeChildren = rg_tree::RedNodeChildren<TomlLanguage>;
pub type SyntaxElementChildren = rg_tree::RedElementChildren<TomlLanguage>;
pub type PreorderWithTokens = rg_tree::PreorderWithTokens<TomlLanguage>;
