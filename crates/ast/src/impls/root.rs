use crate::{support, AstNode};

impl crate::Root {
    pub fn begin_dangling_comments(&self) -> Vec<Vec<crate::BeginDanglingComment>> {
        support::node::begin_dangling_comments(self.syntax().children_with_tokens())
    }

    pub fn end_dangling_comments(&self) -> Vec<Vec<crate::EndDanglingComment>> {
        support::node::end_dangling_comments(self.syntax().children_with_tokens())
    }

    pub fn dangling_comments(&self) -> Vec<Vec<crate::DanglingComment>> {
        support::node::dangling_comments(self.syntax().children_with_tokens())
    }
}
