use super::Format;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DanglingComment(pub ast::Comment);

impl Format for DanglingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}\n", f.ident(), self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeadingComment(pub ast::Comment);

impl Format for LeadingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}\n", f.ident(), self.0)
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
