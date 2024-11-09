use super::Format;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeginDanglingComment(pub ast::Comment);

impl Format for BeginDanglingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}{}", f.ident(), self.0, f.line_ending())
    }
}

impl Format for Vec<BeginDanglingComment> {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        for comment in self {
            comment.fmt(f)?;
        }
        write!(f, "{}", f.line_ending())?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndDanglingComment(pub ast::Comment);

impl Format for EndDanglingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}{}", f.line_ending(), f.ident(), self.0)
    }
}

impl Format for Vec<EndDanglingComment> {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        write!(f, "{}", f.line_ending())?;

        for comment in self {
            comment.fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeadingComment(pub ast::Comment);

impl Format for LeadingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}{}", f.ident(), self.0, f.line_ending())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TailingComment(pub ast::Comment);

impl Format for TailingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}", f.defs().tailing_comment_space(), self.0)
    }
}
