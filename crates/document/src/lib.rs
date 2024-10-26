mod error;
mod key;
mod range;
mod value;

pub use error::Error;
pub use key::Key;
pub use range::Range;
use value::TableKind;
pub use value::{Array, Boolean, Float, Integer, String, Table, Value};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Parsed {
    table: Table,
    errors: Vec<crate::Error>,
}

impl Parsed {
    pub fn document(&self) -> &Table {
        &self.table
    }

    pub fn errors(&self) -> &[crate::Error] {
        &self.errors
    }

    pub fn merge(mut self, other: Parsed) -> Self {
        self.table.merge(other.table);
        self.errors.extend(other.errors.clone());

        self
    }
}

pub trait Parse {
    fn parse(self, source: &str) -> Parsed;
}

impl Parse for ast::Root {
    fn parse(self, source: &str) -> Parsed {
        self.items()
            .map(|item| item.parse(source))
            .into_iter()
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

        let mut value_cursor = &mut Value::Table(Table::new(TableKind::Table));

        if let Some(header) = self.header() {
            let mut keys = header.keys().into_iter();
            while let Some(key) = keys.next() {
                let key = crate::Key::new(source, key);
                if let Value::Table(table) = value_cursor {
                    value_cursor = table
                        .entry(key)
                        .or_insert_with(|| Value::Table(Table::new(TableKind::Table)));
                } else {
                    p.errors.push(crate::Error::DuplicateKey { key });
                }
            }
        }

        if let Value::Table(table) = value_cursor {
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

        let mut value_cursor = &mut Value::Table(Table::new(TableKind::ArrayOfTables));

        if let Some(header) = self.header() {
            let mut keys = header.keys().into_iter();
            while let Some(key) = keys.next() {
                let key = crate::Key::new(source, key);
                if let Value::Table(table) = value_cursor {
                    value_cursor = table
                        .entry(key)
                        .or_insert_with(|| Value::Table(Table::new(TableKind::ArrayOfTables)));
                } else {
                    p.errors.push(crate::Error::DuplicateKey { key });
                }
            }
        }

        if let Value::Table(table) = value_cursor {
            if table.kind() == TableKind::ArrayOfTables && table.entries().is_empty() {
                *value_cursor = Value::Array(Array::new_array_of_tables());
            }
        }

        if let Value::Array(array) = value_cursor {
            let mut table = Table::new(TableKind::ArrayOfTables);
            for kv in self.key_values() {
                p.errors.extend(table.append_key_value(source, kv));
            }

            array.push(Value::Table(table));
        }

        p
    }
}

impl Parse for ast::KeyValue {
    fn parse(self, source: &str) -> Parsed {
        let mut p = Parsed::default();

        p.table.append_key_value(source, self);

        p
    }
}
