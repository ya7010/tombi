use std::{fmt, marker::PhantomData};

use crate::{cursor, NodeOrToken, RedElement, RedNode, SyntaxKind, WalkEvent};

pub trait Language: Sized + Copy + fmt::Debug + Eq + Ord + std::hash::Hash {
    type Kind: Sized + Copy + fmt::Debug + Eq + Ord + std::hash::Hash;

    fn kind_from_raw(raw: SyntaxKind) -> Self::Kind;
    fn kind_to_raw(kind: Self::Kind) -> SyntaxKind;
}

#[derive(Debug, Clone)]
pub struct SyntaxNodeChildren<L: Language> {
    raw: cursor::SyntaxNodeChildren,
    _p: PhantomData<L>,
}

impl<L: Language> Iterator for SyntaxNodeChildren<L> {
    type Item = RedNode<L>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(RedNode::from)
    }
}

impl<L: Language> From<cursor::SyntaxNodeChildren> for SyntaxNodeChildren<L> {
    fn from(raw: cursor::SyntaxNodeChildren) -> Self {
        Self {
            raw,
            _p: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxElementChildren<L: Language> {
    raw: cursor::SyntaxElementChildren,
    _p: PhantomData<L>,
}

impl<L: Language> Iterator for SyntaxElementChildren<L> {
    type Item = RedElement<L>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(NodeOrToken::from)
    }
}

impl<L: Language> From<cursor::SyntaxElementChildren> for SyntaxElementChildren<L> {
    fn from(raw: cursor::SyntaxElementChildren) -> Self {
        Self {
            raw,
            _p: PhantomData,
        }
    }
}

pub struct Preorder<L: Language> {
    raw: cursor::Preorder,
    _p: PhantomData<L>,
}

impl<L: Language> Preorder<L> {
    pub fn skip_subtree(&mut self) {
        self.raw.skip_subtree()
    }
}

impl<L: Language> Iterator for Preorder<L> {
    type Item = WalkEvent<RedNode<L>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|it| it.map(RedNode::from))
    }
}

impl<L: Language> From<cursor::Preorder> for Preorder<L> {
    fn from(raw: cursor::Preorder) -> Self {
        Self {
            raw,
            _p: PhantomData,
        }
    }
}

pub struct PreorderWithTokens<L: Language> {
    raw: cursor::PreorderWithTokens,
    _p: PhantomData<L>,
}

impl<L: Language> PreorderWithTokens<L> {
    pub fn skip_subtree(&mut self) {
        self.raw.skip_subtree()
    }
}

impl<L: Language> Iterator for PreorderWithTokens<L> {
    type Item = WalkEvent<RedElement<L>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|it| it.map(RedElement::from))
    }
}

impl<L: Language> From<cursor::PreorderWithTokens> for PreorderWithTokens<L> {
    fn from(raw: cursor::PreorderWithTokens) -> Self {
        Self {
            raw,
            _p: PhantomData,
        }
    }
}
