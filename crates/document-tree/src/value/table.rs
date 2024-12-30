use ast::{AstChildren, AstNode};
use indexmap::map::Entry;
use indexmap::IndexMap;
use itertools::Itertools;
use toml_version::TomlVersion;

use crate::{support::comment::try_new_comment, Array, Key, TryIntoDocumentTree, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableKind {
    Root,
    Table,
    ParentTable,
    InlineTable,
    ParentKey,
    KeyValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    kind: TableKind,
    range: text::Range,
    symbol_range: text::Range,
    key_values: IndexMap<Key, Value>,
}

impl Table {
    pub(crate) fn new_root(node: &ast::Root) -> Self {
        Self {
            kind: TableKind::Root,
            key_values: Default::default(),
            range: node.syntax().range(),
            symbol_range: node.syntax().range(),
        }
    }

    pub(crate) fn new_table(node: &ast::Table) -> Self {
        Self {
            kind: TableKind::Table,
            key_values: Default::default(),
            range: node.syntax().range(),
            symbol_range: text::Range::new(
                node.bracket_start().unwrap().range().start(),
                node.range().end(),
            ),
        }
    }

    pub(crate) fn new_array_of_tables(node: &ast::ArrayOfTables) -> Self {
        Self {
            kind: TableKind::Table,
            key_values: Default::default(),
            range: node.syntax().range(),
            symbol_range: text::Range::new(
                node.double_bracket_start().unwrap().range().start(),
                node.range().end(),
            ),
        }
    }

    pub(crate) fn new_inline_table(node: &ast::InlineTable) -> Self {
        Self {
            kind: TableKind::InlineTable,
            key_values: Default::default(),
            range: node.syntax().range(),
            symbol_range: text::Range::new(
                node.brace_start().unwrap().range().start(),
                node.brace_end().unwrap().range().end(),
            ),
        }
    }

    pub(crate) fn new_key_value(node: &ast::KeyValue) -> Self {
        Self {
            kind: TableKind::KeyValue,
            key_values: Default::default(),
            range: node.syntax().range(),
            symbol_range: text::Range::new(
                node.keys().unwrap().range().start(),
                node.syntax().range().end(),
            ),
        }
    }

    pub(crate) fn new_parent_table(&self) -> Self {
        Self {
            kind: TableKind::ParentTable,
            key_values: Default::default(),
            range: self.range,
            symbol_range: self.symbol_range,
        }
    }

    pub(crate) fn new_parent_key(&self) -> Self {
        Self {
            kind: TableKind::ParentKey,
            key_values: Default::default(),
            range: self.range,
            symbol_range: self.symbol_range,
        }
    }

    pub fn key_values(&self) -> &IndexMap<Key, Value> {
        &self.key_values
    }

    pub fn merge(&mut self, other: Self) -> Result<(), Vec<crate::Error>> {
        use TableKind::*;
        let mut errors = vec![];

        let is_conflict = match (self.kind, other.kind) {
            (KeyValue, KeyValue) => {
                match self
                    .key_values
                    .values()
                    .zip(other.key_values.values())
                    .next()
                {
                    Some((Value::Table(table1), _)) => table1.kind() == InlineTable,
                    Some((_, Value::Table(table2))) => table2.kind() == InlineTable,
                    Some(_) => false,
                    None => unreachable!("KeyValue must have one value."),
                }
            }
            (Table | InlineTable | KeyValue, Table | InlineTable)
            | (InlineTable, ParentTable | ParentKey | KeyValue)
            | (ParentTable, ParentKey) => true,
            (ParentTable, Table | InlineTable) => {
                self.kind = other.kind;
                false
            }
            (ParentKey, Table | InlineTable) => {
                self.kind = other.kind;
                true
            }
            _ => false,
        };

        if is_conflict {
            errors.push(crate::Error::ConflictTable {
                range1: self.symbol_range,
                range2: other.symbol_range,
            });
            return Err(errors);
        }

        self.range += other.range;
        self.symbol_range += other.symbol_range;

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

    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.key_values.get(key)
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.key_values.get_mut(key)
    }

    #[inline]
    pub fn kind(&self) -> TableKind {
        self.kind
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }

    #[inline]
    pub fn symbol_range(&self) -> text::Range {
        self.symbol_range
    }
}

impl From<Table> for IndexMap<Key, Value> {
    fn from(table: Table) -> IndexMap<Key, Value> {
        table.key_values
    }
}

impl TryIntoDocumentTree<Table> for ast::Table {
    fn try_into_document_tree(self, toml_version: TomlVersion) -> Result<Table, Vec<crate::Error>> {
        let mut table = Table::new_table(&self);
        let mut errors = Vec::new();

        for comment in self.header_leading_comments() {
            if let Err(error) = try_new_comment(comment.as_ref()) {
                errors.push(error);
            }
        }

        if let Some(comment) = self.header_tailing_comment() {
            if let Err(error) = try_new_comment(comment.as_ref()) {
                errors.push(error);
            }
        }

        for key_value in self.key_values() {
            match key_value.try_into_document_tree(toml_version) {
                Ok(other) => {
                    if let Err(errs) = table.merge(other) {
                        errors.extend(errs)
                    }
                }
                Err(errs) => errors.extend(errs),
            }
        }

        let mut keys = get_header_keys(self.header(), toml_version, &mut errors);
        let array_of_table_keys =
            get_array_of_tables_keys(self.array_of_tables_keys(), toml_version, &mut errors);

        let mut is_array_of_table = false;
        while let Some(key) = keys.pop() {
            if is_array_of_table {
                insert_array_of_tables(
                    &mut table,
                    key,
                    Array::new_parent_array_of_tables,
                    &mut errors,
                );
            } else {
                insert_table(
                    &mut table,
                    key,
                    |table| table.new_parent_table(),
                    &mut errors,
                );
            };

            is_array_of_table = array_of_table_keys.contains(&keys);
        }

        if errors.is_empty() {
            Ok(table)
        } else {
            Err(errors)
        }
    }
}

impl TryIntoDocumentTree<Table> for ast::ArrayOfTables {
    fn try_into_document_tree(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> Result<Table, Vec<crate::Error>> {
        let mut table = Table::new_array_of_tables(&self);
        let mut errors = Vec::new();

        for comment in self.header_leading_comments() {
            if let Err(error) = try_new_comment(comment.as_ref()) {
                errors.push(error);
            }
        }

        if let Some(comment) = self.header_tailing_comment() {
            if let Err(error) = try_new_comment(comment.as_ref()) {
                errors.push(error);
            }
        }

        for key_value in self.key_values() {
            match key_value.try_into_document_tree(toml_version) {
                Ok(other) => {
                    if let Err(errs) = table.merge(other) {
                        errors.extend(errs)
                    }
                }
                Err(errs) => errors.extend(errs),
            }
        }

        let mut keys = get_header_keys(self.header(), toml_version, &mut errors);
        let array_of_table_keys =
            get_array_of_tables_keys(self.array_of_tables_keys(), toml_version, &mut errors);

        if let Some(key) = keys.pop() {
            insert_array_of_tables(&mut table, key, Array::new_array_of_tables, &mut errors);
        }

        let mut is_array_of_table = array_of_table_keys.contains(&keys);
        while let Some(key) = keys.pop() {
            if is_array_of_table {
                insert_array_of_tables(
                    &mut table,
                    key,
                    Array::new_parent_array_of_tables,
                    &mut errors,
                );
            } else {
                insert_table(
                    &mut table,
                    key,
                    |table| table.new_parent_table(),
                    &mut errors,
                );
            };

            is_array_of_table = array_of_table_keys.contains(&keys);
        }

        if errors.is_empty() {
            Ok(table)
        } else {
            Err(errors)
        }
    }
}

impl TryIntoDocumentTree<Table> for ast::KeyValue {
    fn try_into_document_tree(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> Result<Table, Vec<crate::Error>> {
        let mut errors = Vec::new();

        for comment in self.leading_comments() {
            if let Err(error) = try_new_comment(comment.as_ref()) {
                errors.push(error);
            }
        }

        let mut keys = get_header_keys(self.keys(), toml_version, &mut errors);

        let value: Value = match self.value().unwrap().try_into_document_tree(toml_version) {
            Ok(value) => value,
            Err(errs) => {
                errors.extend(errs);
                return Err(errors);
            }
        };

        let mut table = if let Some(key) = keys.pop() {
            match Table::new_key_value(&self).insert(key, value) {
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
            match table.new_parent_key().insert(
                key,
                Value::Table(std::mem::replace(&mut table, Table::new_key_value(&self))),
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

impl TryIntoDocumentTree<Table> for ast::InlineTable {
    fn try_into_document_tree(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> Result<Table, Vec<crate::Error>> {
        let mut table = Table::new_inline_table(&self);
        let mut errors = Vec::new();

        for comments in self.inner_begin_dangling_comments() {
            for comment in comments {
                if let Err(error) = try_new_comment(comment.as_ref()) {
                    errors.push(error);
                }
            }
        }

        table.kind = TableKind::Table;

        for (key_value, comma) in self.key_values_with_comma() {
            match key_value.try_into_document_tree(toml_version) {
                Ok(other) => {
                    if let Err(errs) = table.merge(other) {
                        errors.extend(errs)
                    }
                }
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

        table.kind = TableKind::InlineTable;

        for comments in self.inner_end_dangling_comments() {
            for comment in comments {
                if let Err(error) = try_new_comment(comment.as_ref()) {
                    errors.push(error);
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

impl IntoIterator for Table {
    type Item = (Key, Value);
    type IntoIter = indexmap::map::IntoIter<Key, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.key_values.into_iter()
    }
}

fn get_array_of_tables_keys(
    keys_iter: impl Iterator<Item = AstChildren<ast::Key>>,
    toml_version: TomlVersion,
    errors: &mut Vec<crate::Error>,
) -> Vec<Vec<Key>> {
    keys_iter
        .map(|keys| {
            keys.filter_map(|key| match key.try_into_document_tree(toml_version) {
                Ok(key) => Some(key),
                Err(errs) => {
                    errors.extend(errs);
                    None
                }
            })
            .collect_vec()
        })
        .unique()
        .collect_vec()
}

fn get_header_keys(
    header: Option<ast::Keys>,
    toml_version: TomlVersion,
    errors: &mut Vec<crate::Error>,
) -> Vec<Key> {
    header
        .unwrap()
        .keys()
        .filter_map(|key| match key.try_into_document_tree(toml_version) {
            Ok(key) => Some(key),
            Err(errs) => {
                errors.extend(errs);
                None
            }
        })
        .collect_vec()
}

fn insert_table(
    table: &mut Table,
    key: Key,
    new_table_fn: impl Fn(&Table) -> Table,
    errors: &mut Vec<crate::Error>,
) {
    let new_table = new_table_fn(table);
    match new_table_fn(table).insert(key, Value::Table(std::mem::replace(table, new_table))) {
        Ok(t) => *table = t,
        Err(errs) => errors.extend(errs),
    };
}

fn insert_array_of_tables(
    table: &mut Table,
    key: Key,
    new_array_of_tables_fn: impl Fn(&Table) -> Array,
    errors: &mut Vec<crate::Error>,
) {
    let mut array = new_array_of_tables_fn(table);
    let new_table = table.new_parent_table();
    array.push(Value::Table(std::mem::replace(table, new_table)));
    match table.new_parent_table().insert(key, Value::Array(array)) {
        Ok(t) => *table = t,
        Err(errs) => errors.extend(errs),
    };
}
