use std::{marker::PhantomData, sync::Arc};

use ast::AstNode;
use syntax::{SyntaxError, SyntaxNode};

use crate::validation;

#[derive(Debug, PartialEq, Eq)]
pub struct Parsed<T> {
    green_tree: rg_tree::GreenNode,
    errors: Option<Arc<[syntax::SyntaxError]>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Parsed<T> {
    pub fn new(green_tree: rg_tree::GreenNode, errors: Vec<SyntaxError>) -> Parsed<T> {
        Parsed {
            green_tree,
            errors: if errors.is_empty() {
                None
            } else {
                Some(errors.into())
            },
            _ty: PhantomData,
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_tree.clone())
    }

    pub fn into_syntax_node(self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_tree)
    }

    pub fn errors(&self) -> Vec<SyntaxError> {
        let mut errors = if let Some(e) = self.errors.as_deref() {
            e.to_vec()
        } else {
            vec![]
        };
        validation::validate(&self.syntax_node(), &mut errors);
        errors
    }
}

impl<T> Clone for Parsed<T> {
    fn clone(&self) -> Parsed<T> {
        Parsed {
            green_tree: self.green_tree.clone(),
            errors: self.errors.clone(),
            _ty: PhantomData,
        }
    }
}

impl<T: AstNode> Parsed<T> {
    /// Converts this parse result into a parse result for an untyped syntax tree.
    pub fn into_syntax(self) -> Parsed<SyntaxNode> {
        Parsed {
            green_tree: self.green_tree,
            errors: self.errors,
            _ty: PhantomData,
        }
    }

    /// Gets the parsed syntax tree as a typed ast node.
    ///
    /// # Panics
    ///
    /// Panics if the root node cannot be casted into the typed ast node
    /// (e.g. if it's an `ERROR` node).
    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    /// Converts from `Parse<T>` to [`Result<T, Vec<SyntaxError>>`].
    pub fn ok(self) -> Result<T, Vec<SyntaxError>> {
        match self.errors() {
            errors if !errors.is_empty() => Err(errors),
            _ => Ok(self.tree()),
        }
    }
}

impl Parsed<SyntaxNode> {
    pub fn cast<N: AstNode>(self) -> Option<Parsed<N>> {
        if N::cast(self.syntax_node()).is_some() {
            Some(Parsed {
                green_tree: self.green_tree,
                errors: self.errors,
                _ty: PhantomData,
            })
        } else {
            None
        }
    }
}
