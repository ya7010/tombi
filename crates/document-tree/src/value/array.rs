use crate::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrayKind {
    #[default]
    /// An array of tables.
    ///
    /// ```toml
    /// [[array]]
    /// ```
    ArrayOfTables,

    /// An array of tables of parent.
    ///
    /// ```toml
    /// [[array]]
    /// [[array.table]]  # <- Here
    /// ```
    ParentArrayOfTable,

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
    values: Vec<Value>,
}

impl Array {
    pub(crate) fn new_array(node: &ast::Array) -> Self {
        Self {
            kind: ArrayKind::Array,
            values: vec![],
            range: text::Range::new(
                node.bracket_start().unwrap().text_range().start(),
                node.bracket_end().unwrap().text_range().end(),
            ),
        }
    }

    pub(crate) fn new_array_of_tables(table: &crate::Table) -> Self {
        Self {
            kind: ArrayKind::ArrayOfTables,
            values: vec![],
            range: table.range(),
        }
    }

    pub(crate) fn new_parent_array_of_tables(table: &crate::Table) -> Self {
        Self {
            kind: ArrayKind::ParentArrayOfTable,
            values: vec![],
            range: table.range(),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.range += value.range();
        self.values.push(value);
    }

    pub fn merge(&mut self, mut other: Self) -> Result<(), Vec<crate::Error>> {
        use ArrayKind::*;

        let mut errors = Vec::new();

        match (self.kind(), other.kind()) {
            (ParentArrayOfTable, ArrayOfTables) => {
                let Some(Value::Table(table2)) = Into::<Vec<Value>>::into(other).pop() else {
                    unreachable!("Array of tables must have one table.")
                };
                if let Some(Value::Table(table1)) = self.values.last_mut() {
                    if let Err(errs) = table1.merge(table2) {
                        errors.extend(errs);
                    }
                } else {
                    self.push(Value::Table(table2));
                }
            }
            (ArrayOfTables, ParentArrayOfTable) | (ParentArrayOfTable, ParentArrayOfTable) => {
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
            (ArrayOfTables, ArrayOfTables) | (Array, Array) => {
                self.values.extend(other.values);
            }
            (Array, _) | (_, Array) => {
                errors.push(crate::Error::ConflictArray {
                    range1: self.range,
                    range2: other.range,
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
}

impl TryFrom<ast::Array> for Array {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::Array) -> Result<Self, Self::Error> {
        let mut array = Array::new_array(&node);

        let mut errors = Vec::new();

        for value in node.values() {
            match value.try_into() {
                Ok(value) => array.push(value),
                Err(errs) => errors.extend(errs),
            }
        }

        Ok(array)
    }
}

impl Into<Vec<Value>> for Array {
    fn into(self) -> Vec<Value> {
        self.values
    }
}
