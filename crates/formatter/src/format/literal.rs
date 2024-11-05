use syntax::SyntaxToken;

use crate::Format;

mod boolean;
mod datetime;
mod float;
mod integer;
mod string;
use super::comment::{LeadingComment, TailingComment};
use std::fmt::Write;

trait LiteralNode {
    fn token(&self) -> Option<SyntaxToken>;
}

impl<T> Format for T
where
    T: LiteralNode + ast::AstNode,
{
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for comment in self.leading_comments() {
            LeadingComment(comment).fmt(f)?;
        }

        write!(f, "{}{}", f.ident(), self.token().unwrap())?;

        if let Some(comment) = self.tailing_comment() {
            TailingComment(comment).fmt(f)?;
        }

        Ok(())
    }
}
