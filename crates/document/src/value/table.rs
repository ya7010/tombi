use indexmap::map::Entry;
use indexmap::IndexMap;

use crate::Key;
use crate::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableKind {
    #[default]
    Table,
    InlineTable,
    ArrayOfTables,
    DottedKeys,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Table {
    kind: TableKind,
    entries: IndexMap<Key, Value>,
}

impl Table {
    pub fn new(kind: TableKind) -> Self {
        Self {
            kind,
            entries: Default::default(),
        }
    }

    pub(crate) fn append_key_value(
        &mut self,
        source: &str,
        node: ast::KeyValue,
    ) -> Vec<crate::Error> {
        let mut value_cursor = &mut crate::Value::Table(std::mem::take(self));
        let mut errors = vec![];

        if let (Some(keys), Some(value)) = (node.keys(), node.value()) {
            let mut keys = keys.keys().into_iter();
            while let Some(key) = keys.next() {
                let key = crate::Key::new(source, key);
                if let Value::Table(table) = value_cursor {
                    value_cursor = table
                        .entry(key)
                        .or_insert(Value::Table(Table::new(TableKind::DottedKeys)));
                } else {
                    errors.push(crate::Error::DuplicateKey { key });
                }
            }
            *value_cursor = Value::new(source, value);
        }
        errors
    }

    pub fn entries(&self) -> &IndexMap<Key, Value> {
        &self.entries
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        self.entries.insert(key, value);
    }

    pub fn entry(&mut self, key: Key) -> Entry<'_, Key, Value> {
        self.entries.entry(key)
    }

    pub fn merge(mut self, other: Self) -> Self {
        for (key, value) in other.entries {
            self.entries.insert(key, value);
        }
        self
    }

    pub fn kind(&self) -> TableKind {
        self.kind
    }

    pub fn range(&self) -> crate::Range {
        self.entries
            .keys()
            .map(|key| key.range())
            .reduce(|a, b| a.merge(&b))
            .unwrap()
    }
}
