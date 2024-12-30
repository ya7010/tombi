use ast::AstNode;

use crate::{support::comment::try_new_comment, TryIntoDocumentTree, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrayKind {
    #[default]
    /// An array of tables.
    ///
    /// ```toml
    /// [[array]]
    /// ```
    ArrayOfTables,

    /// An array of tables of parent keys.
    ///
    /// ```toml
    /// [[fruit]]
    /// [fruit.info]
    /// #^^^^^                 <- Here
    ///
    /// [[fruit]]
    /// [[fruit.variables]]
    /// # ^^^^^                <- Here
    ///
    /// [fruit.variables.info]
    /// #^^^^^ ^^^^^^^^^       <- Here
    /// ```
    ParentArrayOfTables,

    /// An array.
    ///
    /// ```toml
    /// key = [1, 2, 3]
    /// ```
    Array,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Array {
    kind: ArrayKind,
    range: text::Range,
    symbol_range: text::Range,
    values: Vec<Value>,
}

impl Array {
    pub(crate) fn new_array(node: &ast::Array) -> Self {
        Self {
            kind: ArrayKind::Array,
            values: vec![],
            range: node.range(),
            symbol_range: text::Range::new(
                node.bracket_start().unwrap().range().start(),
                node.bracket_end().unwrap().range().end(),
            ),
        }
    }

    pub(crate) fn new_array_of_tables(table: &crate::Table) -> Self {
        Self {
            kind: ArrayKind::ArrayOfTables,
            values: vec![],
            range: table.range(),
            symbol_range: table.symbol_range(),
        }
    }

    pub(crate) fn new_parent_array_of_tables(table: &crate::Table) -> Self {
        Self {
            kind: ArrayKind::ParentArrayOfTables,
            values: vec![],
            range: table.range(),
            symbol_range: table.symbol_range(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.values.get_mut(index)
    }

    pub fn push(&mut self, value: Value) {
        self.range += value.range();
        self.symbol_range += value.symbol_range();

        self.values.push(value);
    }

    pub fn extend(&mut self, values: Vec<Value>) {
        for value in values {
            self.push(value);
        }
    }

    pub fn merge(&mut self, mut other: Self) -> Result<(), Vec<crate::Error>> {
        use ArrayKind::*;

        let mut errors = Vec::new();

        match (self.kind(), other.kind()) {
            (ArrayOfTables | ParentArrayOfTables, ParentArrayOfTables) => {
                let Some(Value::Table(table2)) = other.values.pop() else {
                    unreachable!("Parent of array of tables must have one table.")
                };
                if let Some(Value::Table(table1)) = self.values.last_mut() {
                    if let Err(errs) = table1.merge(table2) {
                        errors.extend(errs);
                    }
                } else {
                    self.push(Value::Table(table2));
                }
            }
            (ArrayOfTables | ParentArrayOfTables, ArrayOfTables) | (Array, Array) => {
                self.extend(other.values);
            }
            (Array, _) | (_, Array) => {
                errors.push(crate::Error::ConflictArray {
                    range1: self.symbol_range,
                    range2: other.symbol_range,
                });
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn kind(&self) -> ArrayKind {
        self.kind
    }

    pub fn values(&self) -> &[Value] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<Value> {
        &mut self.values
    }

    pub fn range(&self) -> text::Range {
        self.range
    }

    pub fn symbol_range(&self) -> text::Range {
        self.symbol_range
    }
}

impl TryIntoDocumentTree<Array> for ast::Array {
    fn try_into_document_tree(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> Result<Array, Vec<crate::Error>> {
        let mut array = Array::new_array(&self);

        let mut errors = Vec::new();

        for comments in self.inner_begin_dangling_comments() {
            for comment in comments {
                if let Err(error) = try_new_comment(comment.as_ref()) {
                    errors.push(error);
                }
            }
        }

        for (value, comma) in self.values_with_comma() {
            match value.try_into_document_tree(toml_version) {
                Ok(value) => array.push(value),
                Err(errs) => errors.extend(errs),
            }
            if let Some(comma) = comma {
                for comment in comma.leading_comments() {
                    if let Err(error) = try_new_comment(comment.as_ref()) {
                        errors.push(error);
                    }
                }
                if let Some(comment) = comma.tailing_comment() {
                    if let Err(error) = try_new_comment(comment.as_ref()) {
                        errors.push(error);
                    }
                }
            }
        }

        for comments in self.inner_end_dangling_comments() {
            for comment in comments {
                if let Err(error) = try_new_comment(comment.as_ref()) {
                    errors.push(error);
                }
            }
        }

        if errors.is_empty() {
            Ok(array)
        } else {
            Err(errors)
        }
    }
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
