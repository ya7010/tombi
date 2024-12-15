use itertools::Itertools;
use syntax::SyntaxKind;
use syntax::{SyntaxElement, SyntaxKind::*};

use crate::{AstChildren, AstNode, AstToken};

#[inline]
pub fn child<N: AstNode>(parent: &syntax::SyntaxNode) -> Option<N> {
    parent.children().find_map(N::cast)
}

#[inline]
pub fn children<N: AstNode>(parent: &syntax::SyntaxNode) -> AstChildren<N> {
    AstChildren::new(parent)
}

#[inline]
pub fn token(parent: &syntax::SyntaxNode, kind: syntax::SyntaxKind) -> Option<syntax::SyntaxToken> {
    parent
        .children_with_tokens()
        .filter_map(|it| it.into_token())
        .find(|it| it.kind() == kind)
}

#[inline]
pub fn leading_comments<I: Iterator<Item = syntax::SyntaxElement>>(
    iter: I,
) -> impl Iterator<Item = crate::Comment> {
    iter.take_while(|node| matches!(node.kind(), COMMENT | LINE_BREAK | WHITESPACE))
        .filter_map(|node_or_token| match node_or_token {
            SyntaxElement::Token(token) => crate::Comment::cast(token),
            SyntaxElement::Node(_) => None,
        })
}

#[inline]
pub fn tailing_comment<I: Iterator<Item = syntax::SyntaxElement>>(
    iter: I,
    end: syntax::SyntaxKind,
) -> Option<crate::Comment> {
    let mut iter = iter
        .skip_while(|item| item.kind() != end && item.kind() != EOF)
        .skip(1);

    match iter.next()? {
        SyntaxElement::Token(token) if token.kind() == COMMENT => crate::Comment::cast(token),
        SyntaxElement::Token(token) if token.kind() == WHITESPACE => {
            iter.next().and_then(|node_or_token| match node_or_token {
                SyntaxElement::Token(token) if token.kind() == COMMENT => {
                    crate::Comment::cast(token)
                }
                _ => None,
            })
        }
        _ => None,
    }
}

#[inline]
pub fn dangling_comments<I: Iterator<Item = syntax::SyntaxElement>>(
    iter: I,
) -> impl Iterator<Item = crate::Comment> {
    iter.filter_map(|node_or_token| match node_or_token {
        SyntaxElement::Token(token) => crate::Comment::cast(token),
        SyntaxElement::Node(_) => None,
    })
}

#[inline]
pub fn begin_dangling_comments<I: Iterator<Item = syntax::SyntaxElement>>(
    iter: I,
) -> impl Iterator<Item = crate::Comment> {
    iter.take_while(|node| matches!(node.kind(), COMMENT | WHITESPACE | LINE_BREAK))
        .filter_map(|node_or_token| match node_or_token {
            SyntaxElement::Token(token) => crate::Comment::cast(token),
            SyntaxElement::Node(_) => None,
        })
}

#[inline]
pub fn end_dangling_comments<I: Iterator<Item = syntax::SyntaxElement>>(
    iter: I,
) -> impl Iterator<Item = crate::Comment> {
    iter.collect_vec()
        .into_iter()
        .rev()
        .take_while(|node| matches!(node.kind(), COMMENT | WHITESPACE | LINE_BREAK))
        .filter_map(|node_or_token| match node_or_token {
            SyntaxElement::Token(token) => crate::Comment::cast(token),
            SyntaxElement::Node(_) => None,
        })
        .collect_vec()
        .into_iter()
        .rev()
}

#[inline]
pub fn has_inner_comments<I: Iterator<Item = syntax::SyntaxElement>>(
    iter: I,
    start: SyntaxKind,
    end: SyntaxKind,
) -> bool {
    iter.skip_while(|node| node.kind() != start)
        .skip(1)
        .take_while(|node| node.kind() != end)
        .any(|node| {
            node.kind() == COMMENT
                || match node {
                    syntax::SyntaxElement::Node(node) => node
                        .children_with_tokens()
                        .any(|node| node.kind() == COMMENT),
                    _ => false,
                }
        })
}

pub fn prev_siblings_nodes<N: AstNode, T: AstNode>(node: &N) -> impl Iterator<Item = T> {
    node.syntax()
        .siblings(syntax::Direction::Prev)
        .skip(1)
        .filter_map(T::cast)
}

pub fn next_siblings_nodes<N: AstNode, T: AstNode>(node: &N) -> impl Iterator<Item = T> {
    node.syntax()
        .siblings(syntax::Direction::Next)
        .filter_map(T::cast)
}
