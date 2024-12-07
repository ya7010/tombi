use crate::support;
use crate::AstNode;

impl crate::Root {
    pub fn begin_dangling_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::begin_dangling_comments(self.syntax().children_with_tokens())
    }

    pub fn end_dangling_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::end_dangling_comments(self.syntax().children_with_tokens())
    }

    pub fn dangling_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::dangling_comments(self.syntax().children_with_tokens())
    }
}
