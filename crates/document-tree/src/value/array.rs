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

    /// An table.
    ///
    /// ```toml
    /// [[array]]
    /// [[array.table]]  # <- Here
    /// ```
    Table,

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
    values: Vec<Value>,
    range: text::Range,
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

    pub(crate) fn new_array_of_tables(node: &ast::ArrayOfTable) -> Self {
        Self {
            kind: ArrayKind::ArrayOfTables,
            values: vec![],
            range: text::Range::new(
                node.double_bracket_start().unwrap().text_range().start(),
                node.range().end(),
            ),
        }
    }

    pub(crate) fn new_table(node: &ast::Table) -> Self {
        Self {
            kind: ArrayKind::Table,
            values: vec![],
            range: text::Range::new(
                node.bracket_start().unwrap().text_range().start(),
                node.bracket_end().unwrap().text_range().end(),
            ),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn merge(&mut self, other: Self) -> Result<(), Vec<crate::Error>> {
        let mut errors = Vec::new();
        self.range += other.range;

        match (self.kind(), other.kind()) {
            (ArrayKind::ArrayOfTables, ArrayKind::Table) => {
                match (
                    self.values_mut().last_mut().unwrap(),
                    Into::<Vec<Value>>::into(other).pop().unwrap(),
                ) {
                    (Value::Table(table1), Value::Table(table2)) => {
                        if let Err(errs) = table1.merge(table2) {
                            errors.extend(errs);
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            (ArrayKind::ArrayOfTables, ArrayKind::ArrayOfTables)
            | (ArrayKind::Array, ArrayKind::Array) => {
                self.values.extend(other.values);
            }
            _ => {
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
