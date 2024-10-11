mod error;
mod lang;
mod token;

pub use error::Error;
pub use lang::TomlLang;
pub use token::Token;

pub type SyntaxNode = rowan::SyntaxNode<crate::TomlLang>;
#[allow(unused)]
pub type SyntaxToken = rowan::SyntaxToken<crate::TomlLang>;
#[allow(unused)]
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
