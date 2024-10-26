use crate::{Range, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrayKind {
    #[default]
    /// An array of tables.
    ///
    /// ```toml
    /// [[array]]
    /// ```
    ArrayOfTables,

    /// An array.
    ///
    /// ```toml
    /// key = [1, 2, 3]
    /// ```
    Array,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Array {
    kind: ArrayKind,
    elements: Vec<Value>,
    range: Range,
}

impl Array {
    pub fn new_array_of_tables() -> Self {
        Self {
            kind: ArrayKind::ArrayOfTables,
            ..Default::default()
        }
    }
    pub fn new_array() -> Self {
        Self {
            kind: ArrayKind::Array,
            ..Default::default()
        }
    }

    pub fn push(&mut self, value: Value) {
        self.elements.push(value);
    }

    pub fn kind(&self) -> ArrayKind {
        self.kind
    }

    pub fn elements(&self) -> &[Value] {
        &self.elements
    }

    pub fn range(&self) -> Range {
        self.range
    }
}
