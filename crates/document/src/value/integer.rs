use ast::AstNode;

use crate::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerKind {
    Binary,
    Decimal,
    Octal,
    Hexadecimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    kind: IntegerKind,
    value: String,
    range: crate::Range,
}

impl Integer {
    pub fn new_integer_bin(source: &str, node: ast::IntegerBin) -> Self {
        Self {
            kind: IntegerKind::Binary,
            value: node.syntax().to_string(),
            range: Range::from_source(source, node),
        }
    }

    pub fn new_integer_dec(source: &str, node: ast::IntegerDec) -> Self {
        Self {
            kind: IntegerKind::Decimal,
            value: node.syntax().to_string(),
            range: Range::from_source(source, node),
        }
    }

    pub fn new_integer_oct(source: &str, node: ast::IntegerOct) -> Self {
        Self {
            kind: IntegerKind::Octal,
            value: node.syntax().to_string(),
            range: Range::from_source(source, node),
        }
    }

    pub fn new_integer_hex(source: &str, node: ast::IntegerHex) -> Self {
        Self {
            kind: IntegerKind::Hexadecimal,
            value: node.syntax().to_string(),
            range: Range::from_source(source, node),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn kind(&self) -> IntegerKind {
        self.kind
    }

    pub fn range(&self) -> Range {
        self.range
    }
}
