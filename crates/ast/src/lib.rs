pub mod algo;
mod generated;

pub use generated::*;
use std::{fmt::Debug, marker::PhantomData};
use syntax::{NodeOrToken, SyntaxKind::*, T};

pub trait AstNode
where
    Self: Debug,
{
    fn leading_comments(&self) -> Vec<crate::Comment> {
        self.syntax()
            .children_with_tokens()
            .into_iter()
            .take_while(|node| matches!(node.kind(), COMMENT | NEWLINE | WHITESPACE))
            .filter_map(|node_or_token| match node_or_token {
                NodeOrToken::Token(token) => crate::Comment::cast(token),
                NodeOrToken::Node(_) => None,
            })
            .collect()
    }

    fn tailing_comment(&self) -> Option<crate::Comment> {
        self.syntax()
            .last_token()
            .map(crate::Comment::cast)
            .flatten()
    }

    fn can_cast(kind: syntax::SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: syntax::SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &syntax::SyntaxNode;
}

/// Like `AstNode`, but wraps tokens rather than interior nodes.
pub trait AstToken {
    fn can_cast(token: syntax::SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: syntax::SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &syntax::SyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AstChildren<N> {
    inner: syntax::SyntaxNodeChildren,
    ph: PhantomData<N>,
}

impl<N> AstChildren<N> {
    fn new(parent: &syntax::SyntaxNode) -> Self {
        AstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

#[allow(dead_code)]
mod support {
    use super::{AstChildren, AstNode};

    #[inline]
    pub(super) fn child<N: AstNode>(parent: &syntax::SyntaxNode) -> Option<N> {
        parent.children().find_map(N::cast)
    }

    #[inline]
    pub(super) fn children<N: AstNode>(parent: &syntax::SyntaxNode) -> AstChildren<N> {
        AstChildren::new(parent)
    }

    #[inline]
    pub(super) fn token(
        parent: &syntax::SyntaxNode,
        kind: syntax::SyntaxKind,
    ) -> Option<syntax::SyntaxToken> {
        parent
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| it.kind() == kind)
    }
}

impl Root {
    pub fn last_dangling_comments(&self) -> Vec<crate::Comment> {
        self.syntax()
            .children_with_tokens()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .take_while(|node| matches!(node.kind(), COMMENT | NEWLINE))
            .filter_map(|node_or_token| match node_or_token {
                NodeOrToken::Token(token) => crate::Comment::cast(token),
                NodeOrToken::Node(_) => None,
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }
}

impl Table {
    pub fn header_leading_comments(&self) -> Vec<crate::Comment> {
        self.leading_comments()
    }

    pub fn header_tailing_comment(&self) -> Option<crate::Comment> {
        let mut iter = self
            .syntax()
            .children_with_tokens()
            .skip_while(|item| item.kind() != T!(']') && item.kind() != EOF)
            .skip(1);

        match iter.next()? {
            NodeOrToken::Token(token) if token.kind() == COMMENT => crate::Comment::cast(token),
            NodeOrToken::Token(token) if token.kind() == WHITESPACE => {
                iter.next().and_then(|node_or_token| match node_or_token {
                    NodeOrToken::Token(token) if token.kind() == COMMENT => {
                        crate::Comment::cast(token)
                    }
                    _ => None,
                })
            }
            _ => None,
        }
    }
}
