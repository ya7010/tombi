mod error;
mod lang;
mod syntax_kind;

pub use error::Error;
pub use lang::TomlLang;
pub use syntax_kind::SyntaxKind;

pub type SyntaxNode = rowan::SyntaxNode<crate::TomlLang>;
#[allow(unused)]
pub type SyntaxToken = rowan::SyntaxToken<crate::TomlLang>;
#[allow(unused)]
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
