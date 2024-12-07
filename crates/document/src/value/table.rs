use indexmap::map::Entry;
use indexmap::IndexMap;

use crate::{Key, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableKind {
    Table,
    InlineTable,
    KeyValue,
}

impl From<document_tree::TableKind> for TableKind {
    fn from(kind: document_tree::TableKind) -> Self {
        match kind {
            document_tree::TableKind::Root => Self::Table,
            document_tree::TableKind::Table => Self::Table,
            document_tree::TableKind::InlineTable => Self::InlineTable,
            document_tree::TableKind::ArrayOfTables => Self::Table,
            document_tree::TableKind::KeyValue => Self::KeyValue,
            document_tree::TableKind::Tombstone => unreachable!("Tombstone is not supported"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    kind: TableKind,
    key_values: IndexMap<Key, Value>,
}

impl Table {
    pub fn key_values(&self) -> &IndexMap<Key, Value> {
        &self.key_values
    }

    pub fn entry(&mut self, key: Key) -> Entry<'_, Key, Value> {
        self.key_values.entry(key)
    }

    #[inline]
    pub fn kind(&self) -> TableKind {
        self.kind
    }
}

impl From<document_tree::Table> for Table {
    fn from(table: document_tree::Table) -> Self {
        let kind = table.kind().into();
        let key_values = IndexMap::<document_tree::Key, document_tree::Value>::from(table)
            .into_iter()
            .map(|(key, value)| (Key::from(key), Value::from(value)))
            .collect();

        Self { kind, key_values }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.key_values.serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::test_serialize;

    test_serialize! {
        #[test]
        fn array_of_table_table(
            r#"
            [[aaa]]
            key = "value"

            [aaa.bbb]
            ccc = true
            "#
        ) -> Ok(
            json!({
                "aaa": [
                    {
                        "key": "value",
                        "bbb": {
                            "ccc": true
                        }
                    }
                ]
            })
        )
    }

    test_serialize! {
        #[test]
        fn array_of_table_table_twice(
            r#"
            [[aaa]]
            key = "value"

            [aaa.bbb]
            ccc = true

            [[aaa]]
            [aaa.bbb]
            ccc = false
            "#
        ) -> Ok(
            json!({
                "aaa": [
                    {
                        "key": "value",
                        "bbb": {
                            "ccc": true
                        }
                    },
                    {
                        "bbb": {
                            "ccc": false
                        }
                    }
                ]
            })
        )
    }
}
