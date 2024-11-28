mod error;
mod key;
mod node;
mod range;

pub use error::Error;
pub use key::Key;
use node::TableKind;
pub use node::{Array, Boolean, Float, Integer, Node, String, Table};
pub use range::Range;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Document {
    root: Table,
    errors: Vec<crate::Error>,
}

impl Document {
    pub fn document(&self) -> &Table {
        &self.root
    }

    pub fn errors(&self) -> &[crate::Error] {
        &self.errors
    }

    pub fn merge(mut self, other: Document) -> Self {
        self.root.merge(other.root);
        self.errors.extend(other.errors.clone());

        self
    }
}

impl From<Document> for (Table, Vec<crate::Error>) {
    fn from(val: Document) -> Self {
        (val.root, val.errors)
    }
}

pub trait Load {
    fn load(self, source: &str) -> Document;
}

impl Load for ast::Root {
    fn load(self, source: &str) -> Document {
        self.items()
            .map(|item| item.load(source))
            .reduce(|acc, item| acc.merge(item))
            .unwrap_or_default()
    }
}

impl Load for ast::RootItem {
    fn load(self, source: &str) -> Document {
        match self {
            ast::RootItem::Table(table) => table.load(source),
            ast::RootItem::ArrayOfTable(array) => array.load(source),
            ast::RootItem::KeyValue(key_value) => key_value.load(source),
        }
    }
}

impl Load for ast::Table {
    fn load(self, source: &str) -> Document {
        let mut p = Document::default();

        let mut node_cursor = &mut Node::Table(Table::new(TableKind::Table));

        if let Some(header) = self.header() {
            for key in header.keys() {
                let key = crate::Key::new(source, key);
                if let Node::Table(table) = node_cursor {
                    node_cursor = table
                        .entry(key)
                        .or_insert_with(|| Node::Table(Table::new(TableKind::Table)));
                } else {
                    p.errors.push(crate::Error::DuplicateKey { key });
                }
            }
        }

        if let Node::Table(table) = node_cursor {
            for kv in self.key_values() {
                p.errors.extend(table.append_key_value(source, kv));
            }
        }

        p
    }
}

impl Load for ast::ArrayOfTable {
    fn load(self, source: &str) -> Document {
        let mut p = Document::default();

        let mut node_cursor = &mut Node::Table(Table::new(TableKind::ArrayOfTables));

        if let Some(header) = self.header() {
            for key in header.keys() {
                let key = crate::Key::new(source, key);
                if let Node::Table(table) = node_cursor {
                    node_cursor = table
                        .entry(key)
                        .or_insert_with(|| Node::Table(Table::new(TableKind::ArrayOfTables)));
                } else {
                    p.errors.push(crate::Error::DuplicateKey { key });
                }
            }
        }

        if let Node::Table(table) = node_cursor {
            if table.kind() == TableKind::ArrayOfTables && table.entries().is_empty() {
                *node_cursor = Node::Array(Array::new_array_of_tables());
            }
        }

        if let Node::Array(array) = node_cursor {
            let mut table = Table::new(TableKind::ArrayOfTables);
            for kv in self.key_values() {
                p.errors.extend(table.append_key_value(source, kv));
            }

            array.push(Node::Table(table));
        }

        p
    }
}

impl Load for ast::KeyValue {
    fn load(self, source: &str) -> Document {
        let mut p = Document::default();

        p.root.append_key_value(source, self);

        p
    }
}
