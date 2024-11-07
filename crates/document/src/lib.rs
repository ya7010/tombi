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
pub struct Parsed {
    document: Table,
    errors: Vec<crate::Error>,
}

impl Parsed {
    pub fn document(&self) -> &Table {
        &self.document
    }

    pub fn errors(&self) -> &[crate::Error] {
        &self.errors
    }

    pub fn merge(mut self, other: Parsed) -> Self {
        self.document.merge(other.document);
        self.errors.extend(other.errors.clone());

        self
    }
}

impl From<Parsed> for (Table, Vec<crate::Error>) {
    fn from(val: Parsed) -> Self {
        (val.document, val.errors)
    }
}

pub trait Parse {
    fn parse(self, source: &str) -> Parsed;
}

impl Parse for ast::Root {
    fn parse(self, source: &str) -> Parsed {
        self.items()
            .map(|item| item.parse(source))
            .reduce(|acc, item| acc.merge(item))
            .unwrap_or_default()
    }
}

impl Parse for ast::RootItem {
    fn parse(self, source: &str) -> Parsed {
        match self {
            ast::RootItem::Table(table) => table.parse(source),
            ast::RootItem::ArrayOfTable(array) => array.parse(source),
            ast::RootItem::KeyValue(key_value) => key_value.parse(source),
        }
    }
}

impl Parse for ast::Table {
    fn parse(self, source: &str) -> Parsed {
        let mut p = Parsed::default();

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

impl Parse for ast::ArrayOfTable {
    fn parse(self, source: &str) -> Parsed {
        let mut p = Parsed::default();

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

impl Parse for ast::KeyValue {
    fn parse(self, source: &str) -> Parsed {
        let mut p = Parsed::default();

        p.document.append_key_value(source, self);

        p
    }
}
