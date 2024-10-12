use std::marker::PhantomData;

pub trait AstNode {
    fn cast(syntax: syntax::SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &syntax::SyntaxNode;
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
