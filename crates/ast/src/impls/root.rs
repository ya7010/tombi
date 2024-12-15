use crate::support;
use crate::AstNode;

impl crate::Root {
    pub fn begin_dangling_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::node::begin_dangling_comments(self.syntax().children_with_tokens())
    }

    pub fn end_dangling_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::node::end_dangling_comments(self.syntax().children_with_tokens())
    }

    pub fn dangling_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::node::dangling_comments(self.syntax().children_with_tokens())
    }
}
