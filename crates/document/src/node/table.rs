use indexmap::map::Entry;
use indexmap::IndexMap;

use crate::Key;
use crate::Node;

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
    entries: IndexMap<Key, Node>,
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
        let mut node_cursor = &mut crate::Node::Table(std::mem::take(self));
        let mut errors = vec![];

        if let (Some(keys), Some(value)) = (node.keys(), node.value()) {
            for key in keys.keys() {
                let key = crate::Key::new(source, key);
                if let Node::Table(table) = node_cursor {
                    node_cursor = table
                        .entry(key)
                        .or_insert(Node::Table(Table::new(TableKind::DottedKeys)));
                } else {
                    errors.push(crate::Error::DuplicateKey { key });
                }
            }
            *node_cursor = Node::new(source, value);
        }
        errors
    }

    pub fn entries(&self) -> &IndexMap<Key, Node> {
        &self.entries
    }

    pub fn insert(&mut self, key: Key, node: Node) {
        self.entries.insert(key, node);
    }

    pub fn entry(&mut self, key: Key) -> Entry<'_, Key, Node> {
        self.entries.entry(key)
    }

    pub fn merge(&mut self, other: Self) -> Vec<crate::Error> {
        let mut errors = vec![];
        // Merge the entries of the two tables recursively
        for (key, value2) in other.entries {
            match self.entries.entry(key.clone()) {
                Entry::Occupied(mut entry) => {
                    let value1 = entry.get_mut();
                    match (value1, value2) {
                        (Node::Table(table1), Node::Table(table2)) => {
                            table1.merge(table2);
                        }
                        _ => {
                            errors.push(crate::Error::DuplicateKey { key: key.clone() });
                        }
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(value2);
                }
            }
        }
        errors
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
