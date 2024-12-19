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
            document_tree::TableKind::ParentHeader => Self::Table,
            document_tree::TableKind::LastHeader => Self::Table,
            document_tree::TableKind::InlineTable => Self::InlineTable,
            document_tree::TableKind::KeyValue => Self::KeyValue,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    kind: TableKind,
    key_values: IndexMap<Key, Value>,
}

impl Table {
    pub(crate) fn new(kind: TableKind) -> Self {
        Self {
            kind,
            key_values: IndexMap::new(),
        }
    }

    #[inline]
    pub fn kind(&self) -> TableKind {
        self.kind
    }

    #[inline]
    pub fn key_values(&self) -> &IndexMap<Key, Value> {
        &self.key_values
    }

    pub fn entry(&mut self, key: Key) -> Entry<'_, Key, Value> {
        self.key_values.entry(key)
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        self.key_values.insert(key, value);
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

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let key_values = IndexMap::<Key, Value>::deserialize(deserializer)?;
        Ok(Self {
            kind: TableKind::Table,
            key_values,
        })
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::test_serialize;

    test_serialize!(
        #[test]
        fn key_value(
            r#"
            key = "value"
            "#
        ) -> Ok(json!({"key": "value"}))
    );

    test_serialize!(
        #[test]
        fn keys_value(
            r#"
            foo.bar.baz = "value"
            "#
        ) -> Ok(json!(
            {
                "foo": {
                    "bar": {
                        "baz": "value"
                    }
                }
            }
        ))
    );

    test_serialize!(
        #[test]
        fn table(
            r#"
            [foo]
            bar = "value"
            baz = 42
            "#
        ) -> Ok(json!({"foo": {"bar": "value", "baz": 42}}))
    );

    test_serialize! {
        #[test]
        fn tables(
            r#"
            [foo1]
            bar = 1

            [foo2]
            bar = 2
            "#
        ) -> Ok(json!({
            "foo1": {"bar": 1},
            "foo2": {"bar": 2}
        }))
    }

    test_serialize! {
        #[test]
        fn sub_empty(
            r#"
            [a]
            [a.b]
            "#
        ) -> Ok(json!({ "a": { "b": {} } }))
    }

    test_serialize! {
        #[test]
        fn key_dotted_3(
            r#"
            [tbl]
            a.b.c = {d.e=1}

            [tbl.x]
            a.b.c = {d.e=1}
            "#
        ) -> Ok(json!(
            {
                "tbl": {
                    "a": { "b": { "c": { "d": { "e": 1 } } } },
                    "x": { "a": { "b": { "c": { "d": { "e": 1 } } } } }
                }
            }
        ))
    }

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

    test_serialize! {
        #[test]
        fn array_of_table_table_twice_with_key(
            r#"
            a.b=0
            # Since table "a" is already defined, it can't be replaced by an inline table.
            a={}
       "#
        ) -> Err([
            ("conflicting table", ((0, 0), (0, 5)))
        ])
    }
}
