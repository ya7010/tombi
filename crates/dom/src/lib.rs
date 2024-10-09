mod error;
mod node;

pub use error::Error;
pub use node::Node;

pub trait TryFromSyntax<'a> {
    fn try_from_syntax(syntax: &'a lexer::SyntaxElement) -> Result<Self, Vec<crate::Error>>
    where
        Self: Sized;
}
