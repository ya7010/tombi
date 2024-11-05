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
    use super::*;

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

    #[inline]
    pub(super) fn header_leading_comments<N: AstNode>(node: &N) -> Vec<crate::Comment> {
        node.leading_comments()
    }

    #[inline]
    pub(super) fn header_tailing_comment<N: AstNode>(
        node: &N,
        end: syntax::SyntaxKind,
    ) -> Option<crate::Comment> {
        let mut iter = node
            .syntax()
            .children_with_tokens()
            .skip_while(|item| item.kind() != end && item.kind() != EOF)
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

    #[inline]
    pub(super) fn begin_dangling_comments<I: Iterator<Item = syntax::NodeOrToken>>(
        iter: I,
    ) -> Vec<crate::Comment> {
        iter.take_while(|node| matches!(node.kind(), COMMENT | WHITESPACE | NEWLINE))
            .filter_map(|node_or_token| match node_or_token {
                NodeOrToken::Token(token) => crate::Comment::cast(token),
                NodeOrToken::Node(_) => None,
            })
            .collect()
    }

    #[inline]
    pub(super) fn end_dangling_comments<I: Iterator<Item = syntax::NodeOrToken>>(
        iter: I,
    ) -> Vec<crate::Comment> {
        iter.collect::<Vec<_>>()
            .into_iter()
            .rev()
            .take_while(|node| matches!(node.kind(), COMMENT | WHITESPACE | NEWLINE))
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

impl Root {
    pub fn begin_dangling_comments(&self) -> Vec<crate::Comment> {
        support::begin_dangling_comments(self.syntax().children_with_tokens())
    }

    pub fn end_dangling_comments(&self) -> Vec<crate::Comment> {
        support::end_dangling_comments(self.syntax().children_with_tokens())
    }
}

impl Table {
    pub fn header_leading_comments(&self) -> Vec<crate::Comment> {
        support::header_leading_comments(self)
    }

    pub fn header_tailing_comment(&self) -> Option<crate::Comment> {
        support::header_tailing_comment(self, T!(']'))
    }
}

impl ArrayOfTable {
    pub fn header_leading_comments(&self) -> Vec<crate::Comment> {
        support::header_leading_comments(self)
    }

    pub fn header_tailing_comment(&self) -> Option<crate::Comment> {
        support::header_tailing_comment(self, T!("]]"))
    }
}

impl Array {
    pub fn inner_begin_dangling_comments(&self) -> Vec<crate::Comment> {
        support::begin_dangling_comments(
            self.syntax()
                .children_with_tokens()
                .skip_while(|node| node.kind() != T!('['))
                .skip(1),
        )
    }

    pub fn inner_end_dangling_comments(&self) -> Vec<crate::Comment> {
        support::end_dangling_comments(
            self.syntax()
                .children_with_tokens()
                .into_iter()
                .take_while(|node| node.kind() != T!(']')),
        )
    }

    pub fn has_inner_comments(&self) -> bool {
        self.syntax()
            .children_with_tokens()
            .skip_while(|node| node.kind() != T!('['))
            .skip(1)
            .any(|node| node.kind() == COMMENT)
    }

    pub fn has_tailing_comma_after_last_element(&self) -> bool {
        self.syntax()
            .children_with_tokens()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .skip_while(|item| item.kind() != T!(']'))
            .skip(1)
            .skip_while(|item| matches!(item.kind(), WHITESPACE | COMMENT | NEWLINE))
            .next()
            .map_or(false, |it| it.kind() == T!(,))
    }
}

/// Matches a `SyntaxNode` against an `ast` type.
///
/// # Example:
///
/// ```ignore
/// match_ast! {
///     match node {
///         ast::CallExpr(it) => { ... },
///         ast::MethodCallExpr(it) => { ... },
///         ast::MacroCall(it) => { ... },
///         _ => None,
///     }
/// }
/// ```
#[macro_export]
macro_rules! match_ast {
    (match $node:ident { $($tt:tt)* }) => { $crate::match_ast!(match ($node) { $($tt)* }) };

    (match ($node:expr) {
        $( $( $path:ident )::+ ($it:pat) => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = $($path::)+cast($node.clone()) { $res } else )*
        { $catch_all }
    }};
}
