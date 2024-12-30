mod array;
mod boolean;
mod date_time;
mod float;
mod inline_table;
mod integer;
mod string;

use crate::Format;
use std::fmt::Write;
use syntax::SyntaxToken;

impl Format for ast::Value {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Array(it) => it.fmt(f),
            Self::BasicString(it) => it.fmt(f),
            Self::Boolean(it) => it.fmt(f),
            Self::Float(it) => it.fmt(f),
            Self::InlineTable(it) => it.fmt(f),
            Self::IntegerBin(it) => it.fmt(f),
            Self::IntegerDec(it) => it.fmt(f),
            Self::IntegerHex(it) => it.fmt(f),
            Self::IntegerOct(it) => it.fmt(f),
            Self::LiteralString(it) => it.fmt(f),
            Self::LocalDate(it) => it.fmt(f),
            Self::LocalDateTime(it) => it.fmt(f),
            Self::LocalTime(it) => it.fmt(f),
            Self::MultiLineBasicString(it) => it.fmt(f),
            Self::MultiLineLiteralString(it) => it.fmt(f),
            Self::OffsetDateTime(it) => it.fmt(f),
        }
    }
}

trait LiteralNode {
    fn token(&self) -> Option<SyntaxToken>;
}

impl<T> Format for T
where
    T: LiteralNode + ast::AstNode,
{
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for comment in self.leading_comments() {
            comment.fmt(f)?;
        }

        f.write_indent()?;
        write!(f, "{}", self.token().unwrap())?;

        if let Some(comment) = self.tailing_comment() {
            comment.fmt(f)?;
        }

        Ok(())
    }
}
