mod error;
mod node;

pub use error::Error;
pub use node::Node;

pub trait FromSyntax<'a> {
    fn from_syntax(syntax: &'a lexer::SyntaxElement) -> Result<Self, Vec<crate::Error>>
    where
        Self: Sized;
}
