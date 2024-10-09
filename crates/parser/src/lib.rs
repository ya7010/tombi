mod error;
mod lang;
pub mod parser;
mod token;

pub use error::Error;
pub use lang::TomlLang;
pub use token::Token;

type SyntaxNode = rowan::SyntaxNode<TomlLang>;
#[allow(unused)]
type SyntaxToken = rowan::SyntaxToken<TomlLang>;
#[allow(unused)]
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
