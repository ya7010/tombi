use syntax::SyntaxToken;

use crate::Format;

mod boolean;
mod datetime;
mod float;
mod integer;
mod string;
use super::comment::TailingComment;
use std::fmt::Write;

trait LiteralNode {
    fn token(&self) -> Option<SyntaxToken>;
}

impl<T> Format for T
where
    T: LiteralNode + ast::AstNode,
{
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.token().unwrap())?;
        if let Some(comment) = self.tailing_comment() {
            TailingComment(comment).fmt(f)?;
        }

        Ok(())
    }
}
