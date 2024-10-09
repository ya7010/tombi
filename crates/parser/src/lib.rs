mod error;
mod lang;
pub mod parser;

pub use error::Error;
pub use lang::TomlLang;
pub use lexer::Token;

type SyntaxNode = rowan::SyntaxNode<TomlLang>;
#[allow(unused)]
type SyntaxToken = rowan::SyntaxToken<TomlLang>;
#[allow(unused)]
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
