mod error;
mod key;
mod range;
mod value;

pub use error::Error;
pub use key::Key;
pub use range::Range;
pub use value::{Array, Boolean, Float, Integer, String, Table, Value};

pub trait TryIntoTable {
    fn try_into_table(self, source: &str) -> Result<Table, crate::Error>;
}

// impl TryIntoTable for ast::Root {
//     fn try_into_table(self, source: &str) -> Result<Table, crate::Error> {
//         self.items()
//             .map(|item| item.try_into_table(source))
//             .into_iter()
//             .reduce(|acc, item| acc.merge(item))
//             .unwrap_or_default()
//     }
// }

// impl TryIntoTable for ast::RootItem {
//     fn try_into_table(self, source: &str) -> Result<Table, crate::Error> {
//         match self {
//             ast::RootItem::Table(table) => table.try_into_table(source),
//             ast::RootItem::ArrayOfTable(array) => array.try_into_table(source),
//             ast::RootItem::KeyValue(key_value) => key_value.try_into_table(source),
//         }
//     }
// }

// impl TryIntoTable for ast::Table {
//     fn try_into_table(self, source: &str) -> Result<Table, crate::Error> {
//         let mut table = Table::default();

//         if let Some(header) = self.header() {
//             header.keys().fold(&mut table, |tmp_table, key| {
//                 let key = Key::new(source, key);
//                 let value = tmp_table
//                     .entry(key)
//                     .or_insert(Value::Table(Table::default()));
//                 match value {
//                     Value::Table(table) => table,
//                     _ => return Err(crate::Error::DuplicateKey { key: key.clone() }),
//                 }
//             });
//         }
//         Ok(table)
//     }
// }
