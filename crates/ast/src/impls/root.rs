use syntax::{SyntaxElement, SyntaxKind};

use crate::{support, AstNode};

impl crate::Root {
    pub fn key_values_begin_dangling_comments(&self) -> Vec<Vec<crate::BeginDanglingComment>> {
        support::node::begin_dangling_comments(self.syntax().children_with_tokens())
    }

    pub fn key_values_end_dangling_comments(&self) -> Vec<Vec<crate::EndDanglingComment>> {
        support::node::end_dangling_comments(self.syntax().children_with_tokens().take_while(
            |node_or_token| match node_or_token {
                SyntaxElement::Node(node) => node.kind() != SyntaxKind::TABLE,
                _ => true,
            },
        ))
    }

    pub fn dangling_comments(&self) -> Vec<Vec<crate::DanglingComment>> {
        support::node::dangling_comments(self.syntax().children_with_tokens())
    }
}
