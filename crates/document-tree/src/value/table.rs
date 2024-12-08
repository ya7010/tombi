use ast::AstNode;
use indexmap::map::Entry;
use indexmap::IndexMap;
use itertools::Itertools;

use crate::{Array, Key, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableKind {
    Root,
    Table,
    InlineTable,
    ArrayOfTables,
    KeyValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    kind: TableKind,
    key_values: IndexMap<Key, Value>,
    range: text::Range,
}

impl Table {
    pub(crate) fn new_root(node: &ast::Root) -> Self {
        Self {
            kind: TableKind::Root,
            key_values: Default::default(),
            range: node.syntax().text_range(),
        }
    }

    pub(crate) fn new_table(node: &ast::Table) -> Self {
        Self {
            kind: TableKind::Table,
            key_values: Default::default(),
            range: text::Range::new(
                node.bracket_start().unwrap().text_range().start(),
                node.range().end(),
            ),
        }
    }

    pub(crate) fn new_array_of_tables(node: &ast::ArrayOfTables) -> Self {
        Self {
            kind: TableKind::ArrayOfTables,
            key_values: Default::default(),
            range: text::Range::new(
                node.double_bracket_start().unwrap().text_range().start(),
                node.range().end(),
            ),
        }
    }

    pub(crate) fn new_inline_table(node: &ast::InlineTable) -> Self {
        Self {
            kind: TableKind::InlineTable,
            key_values: Default::default(),
            range: text::Range::new(
                node.brace_start().unwrap().text_range().start(),
                node.brace_end().unwrap().text_range().end(),
            ),
        }
    }

    pub(crate) fn new_key_value(node: &ast::KeyValue) -> Self {
        Self {
            kind: TableKind::KeyValue,
            key_values: Default::default(),
            range: text::Range::new(
                node.keys().unwrap().range().start(),
                node.syntax().text_range().end(),
            ),
        }
    }

    pub(crate) fn new_parent(&self) -> Self {
        Self {
            kind: self.kind,
            key_values: Default::default(),
            range: self.range,
        }
    }

    pub fn key_values(&self) -> &IndexMap<Key, Value> {
        &self.key_values
    }

    pub fn merge(&mut self, other: Self) -> Result<(), Vec<crate::Error>> {
        let mut errors = vec![];
        self.range += other.range;
        // Merge the key_values of the two tables recursively
        for (key, value2) in other.key_values {
            match self.key_values.entry(key.clone()) {
                Entry::Occupied(mut entry) => {
                    let value1 = entry.get_mut();
                    match (value1, value2) {
                        (Value::Table(table1), Value::Table(table2)) => {
                            if let Err(errs) = table1.merge(table2) {
                                errors.extend(errs);
                            };
                        }
                        (Value::Array(array1), Value::Array(array2)) => {
                            if let Err(errs) = array1.merge(array2) {
                                errors.extend(errs);
                            }
                        }
                        _ => {
                            errors.push(crate::Error::DuplicateKey {
                                key: key.value().to_string(),
                                range: key.range(),
                            });
                        }
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(value2);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub(crate) fn insert(mut self, key: Key, value: Value) -> Result<Self, Vec<crate::Error>> {
        let mut errors = Vec::new();

        match self.key_values.entry(key) {
            Entry::Occupied(mut entry) => {
                let existing_value = entry.get_mut();
                match (existing_value, value) {
                    (Value::Table(table1), Value::Table(table2)) => {
                        if let Err(errs) = table1.merge(table2) {
                            errors.extend(errs);
                        }
                    }
                    (Value::Array(array1), Value::Array(array2)) => {
                        if let Err(errs) = array1.merge(array2) {
                            errors.extend(errs);
                        }
                    }
                    _ => {
                        errors.push(crate::Error::DuplicateKey {
                            key: entry.key().value().to_string(),
                            range: entry.key().range(),
                        });
                    }
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(value);
            }
        }

        if errors.is_empty() {
            Ok(self)
        } else {
            Err(errors)
        }
    }

    pub fn entry(&mut self, key: Key) -> Entry<'_, Key, Value> {
        self.key_values.entry(key)
    }

    #[inline]
    pub fn kind(&self) -> TableKind {
        self.kind
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl From<Table> for IndexMap<Key, Value> {
    fn from(table: Table) -> IndexMap<Key, Value> {
        table.key_values
    }
}

impl TryFrom<ast::Table> for Table {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::Table) -> Result<Self, Self::Error> {
        let mut table = Table::new_table(&node);
        let mut errors = Vec::new();

        let array_of_table_keys = node
            .array_of_tables_keys()
            .map(|keys| keys.map(|key| Key::from(key)).collect_vec())
            .unique()
            .collect_vec();

        for key_value in node.key_values() {
            match key_value.try_into() {
                Ok(other) => {
                    if let Err(errs) = table.merge(other) {
                        errors.extend(errs)
                    }
                }
                Err(errs) => errors.extend(errs),
            }
        }

        let mut is_array_of_table = false;
        let mut keys = node
            .header()
            .unwrap()
            .keys()
            .map(|key| Key::from(key))
            .collect_vec();
        while let Some(key) = keys.pop() {
            let result: Result<Table, Vec<crate::Error>> = if is_array_of_table {
                let mut array = Array::new_table(&node);
                let new_table = table.new_parent();
                array.push(Value::Table(std::mem::replace(&mut table, new_table)));

                table.new_parent().insert(key, Value::Array(array))
            } else {
                let new_table = table.new_parent();
                table
                    .new_parent()
                    .insert(key, Value::Table(std::mem::replace(&mut table, new_table)))
            };

            is_array_of_table = array_of_table_keys.contains(&keys);

            match result {
                Ok(t) => table = t,
                Err(errs) => {
                    errors.extend(errs);
                }
            }
        }

        if errors.is_empty() {
            Ok(table)
        } else {
            Err(errors)
        }
    }
}

impl TryFrom<ast::ArrayOfTables> for Table {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::ArrayOfTables) -> Result<Self, Self::Error> {
        let mut table = Table::new_array_of_tables(&node);
        let mut errors = Vec::new();

        for key_value in node.key_values() {
            match key_value.try_into() {
                Ok(other) => {
                    if let Err(errs) = table.merge(other) {
                        errors.extend(errs)
                    }
                }
                Err(errs) => errors.extend(errs),
            }
        }
        let mut keys = node.header().unwrap().keys().rev().map(Key::from);

        if let Some(key) = keys.next() {
            let mut array = Array::new_array_of_tables(&node);

            let new_table = table.new_parent();
            array.push(Value::Table(std::mem::replace(&mut table, new_table)));
            table = table.new_parent().insert(key, Value::Array(array)).unwrap();
        }

        for key in keys {
            match table.new_parent().insert(
                key,
                Value::Table(std::mem::replace(
                    &mut table,
                    Table::new_array_of_tables(&node),
                )),
            ) {
                Ok(t) => table = t,
                Err(errs) => {
                    errors.extend(errs);
                }
            }
        }

        if errors.is_empty() {
            Ok(table)
        } else {
            Err(errors)
        }
    }
}

impl TryFrom<ast::KeyValue> for Table {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::KeyValue) -> Result<Table, Self::Error> {
        let mut errors = Vec::new();
        let mut keys = node
            .keys()
            .unwrap()
            .keys()
            .map(Key::from)
            .collect_vec();

        let value: Value = match node.value().unwrap().try_into() {
            Ok(value) => value,
            Err(errs) => {
                errors.extend(errs);
                return Err(errors);
            }
        };

        let mut table = if let Some(key) = keys.pop() {
            match Table::new_key_value(&node).insert(key, value) {
                Ok(table) => table,
                Err(errs) => {
                    errors.extend(errs);
                    return Err(errors);
                }
            }
        } else {
            return Err(errors);
        };

        for key in keys.into_iter().rev() {
            match table.new_parent().insert(
                key,
                Value::Table(std::mem::replace(&mut table, Table::new_key_value(&node))),
            ) {
                Ok(t) => table = t,
                Err(errs) => {
                    errors.extend(errs);
                }
            }
        }

        if errors.is_empty() {
            Ok(table)
        } else {
            Err(errors)
        }
    }
}

impl TryFrom<ast::InlineTable> for Table {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::InlineTable) -> Result<Self, Self::Error> {
        let mut table = Table::new_inline_table(&node);
        let mut errors = Vec::new();

        for key_value in node.key_values() {
            match key_value.try_into() {
                Ok(other) => {
                    if let Err(errs) = table.merge(other) {
                        errors.extend(errs)
                    }
                }
                Err(errs) => errors.extend(errs),
            }
        }

        if errors.is_empty() {
            Ok(table)
        } else {
            Err(errors)
        }
    }
}
