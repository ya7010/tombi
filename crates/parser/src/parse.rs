use std::{marker::PhantomData, sync::Arc};

use ast::AstNode;
use syntax::{SyntaxError, SyntaxNode};

use crate::validation;

#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: rowan::GreenNode,
    errors: Option<Arc<[syntax::SyntaxError]>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Parse<T> {
    pub fn new(green: rowan::GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
        Parse {
            green,
            errors: if errors.is_empty() {
                None
            } else {
                Some(errors.into())
            },
            _ty: PhantomData,
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn into_syntax_node(self) -> SyntaxNode {
        SyntaxNode::new_root(self.green)
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

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse {
            green: self.green.clone(),
            errors: self.errors.clone(),
            _ty: PhantomData,
        }
    }
}

impl<T: AstNode> Parse<T> {
    /// Converts this parse result into a parse result for an untyped syntax tree.
    pub fn to_syntax(self) -> Parse<SyntaxNode> {
        Parse {
            green: self.green,
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

impl Parse<SyntaxNode> {
    pub fn cast<N: AstNode>(self) -> Option<Parse<N>> {
        if N::cast(self.syntax_node()).is_some() {
            Some(Parse {
                green: self.green,
                errors: self.errors,
                _ty: PhantomData,
            })
        } else {
            None
        }
    }
}
