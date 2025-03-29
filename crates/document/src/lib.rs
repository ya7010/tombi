mod key;
mod value;

use std::ops::Deref;

pub use key::{Key, KeyKind};
use toml_version::TomlVersion;
pub use value::{
    Array, ArrayKind, Boolean, Float, Integer, IntegerKind, LocalDate, LocalDateTime, LocalTime,
    OffsetDateTime, String, StringKind, Table, TableKind, Value,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Document(pub(crate) Table);

impl From<Document> for Table {
    fn from(document: Document) -> Self {
        document.0
    }
}

pub trait IntoDocument<T> {
    fn into_document(self, toml_version: TomlVersion) -> T;
}

impl IntoDocument<Document> for document_tree::DocumentTree {
    fn into_document(self, toml_version: TomlVersion) -> Document {
        Document(document_tree::Table::from(self).into_document(toml_version))
    }
}

impl Deref for Document {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Document {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Document {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let table = Table::deserialize(deserializer)?;
        Ok(Document(table))
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! test_deserialize {
    {#[test] fn $name:ident($source:expr) -> Ok($json:expr)} => {
        test_deserialize! {#[test] fn $name($source, toml_version::TomlVersion::default()) -> Ok($json)}
    };

    {#[test] fn $name:ident($source:expr, $toml_version:expr) -> Ok($json:expr)} => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use ast::AstNode;
            use itertools::Itertools;
            use document_tree::IntoDocumentTreeAndErrors;
            use $crate::IntoDocument;

            let source = textwrap::dedent($source);
            let p = parser::parse(&source.trim());
            pretty_assertions::assert_eq!(p.errors($toml_version).collect_vec(), Vec::<&parser::Error>::new());
            let root = ast::Root::cast(p.into_syntax_node()).unwrap();
            let (document_tree, errors) = root.into_document_tree_and_errors($toml_version).into();
            pretty_assertions::assert_eq!(errors, vec![]);
            let document: $crate::Document = document_tree.into_document($toml_version);
            let serialized = serde_json::to_string(&document).unwrap();
            pretty_assertions::assert_eq!(serialized, $json.to_string());
        }
    };

    {#[test] fn $name:ident($source:expr) -> Err($errors:expr)} => {
        test_deserialize! {#[test] fn $name($source, toml_version::TomlVersion::default()) -> Err($errors)}
    };

    {#[test] fn $name:ident($source:expr, $toml_version:expr) -> Err($errors:expr)} => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use ast::AstNode;
            use itertools::Itertools;
            use document_tree::IntoDocumentTreeAndErrors;

            let source = textwrap::dedent($source);
            let p = parser::parse(&source.trim());
            let expected_errors = $errors
                .into_iter()
                .map(|(m, r)| (m.to_string(), text::Range::from(r)))
                .collect_vec();

            let errors = p.errors($toml_version).collect_vec();
            if !errors.is_empty() {
                pretty_assertions::assert_eq!(
                    errors
                        .iter()
                        .map(|e| (e.to_message(), e.range()))
                        .collect_vec(),
                    expected_errors,
                );
            }
            let root = ast::Root::cast(p.into_syntax_node()).unwrap();
            let (_, errs) = root.into_document_tree_and_errors($toml_version).into();
            pretty_assertions::assert_eq!(
                errs
                    .iter()
                    .map(|e| (e.to_message(), e.range()))
                    .collect_vec(),
                expected_errors
            );
        }
    };
}

#[cfg(test)]
mod test {
    use serde_json::json;

    test_deserialize! {
        #[test]
        fn empty("") -> Ok(json!({}))
    }

    test_deserialize! {
        #[test]
        fn key_values(
            r#"
            key = "value"
            flag = true
            "#
        ) -> Ok(json!({"key": "value", "flag": true}))
    }

    test_deserialize! {
        #[test]
        fn array_of_tables_sample(
            r#"
            [[fruits]]
            name = "apple"

            [fruits.physical]  # subtable
            color = "red"
            shape = "round"

            [[fruits.varieties]]  # nested array of tables
            name = "red delicious"

            [[fruits.varieties]]
            name = "granny smith"

            [[fruits]]
            name = "banana"

            [[fruits.varieties]]
            name = "plantain"
            "#
        ) -> Ok(
            json!(
                {
                    "fruits": [
                        {
                            "name": "apple",
                            "physical": {
                                "color": "red",
                                "shape": "round"
                            },
                            "varieties": [
                                { "name": "red delicious" },
                                { "name": "granny smith" }
                            ]
                        },
                        {
                            "name": "banana",
                            "varieties": [
                                { "name": "plantain" }
                            ]
                        }
                    ]
                }
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_document_serialization() {
        // Create a test document with various value types
        let mut table = Table::new(TableKind::Table);

        // Add string value
        table.insert(
            Key::new(KeyKind::BareKey, "string".to_string()),
            Value::String(String::new(StringKind::BasicString, "hello".to_string())),
        );

        // Add integer value
        table.insert(
            Key::new(KeyKind::BareKey, "integer".to_string()),
            Value::Integer(Integer::new(42)),
        );

        // Add float value
        table.insert(
            Key::new(KeyKind::BareKey, "float".to_string()),
            Value::Float(Float::new(3.14)),
        );

        // Add boolean value
        table.insert(
            Key::new(KeyKind::BareKey, "boolean".to_string()),
            Value::Boolean(Boolean::new(true)),
        );

        // Add array value
        let mut array = Array::new(ArrayKind::Array);
        array.push(Value::Integer(Integer::new(1)));
        array.push(Value::Integer(Integer::new(2)));
        array.push(Value::Integer(Integer::new(3)));
        table.insert(
            Key::new(KeyKind::BareKey, "array".to_string()),
            Value::Array(array),
        );

        // Create document
        let document = Document(table);

        // Serialize to JSON
        let serialized = serde_json::to_string(&document).unwrap();

        // Expected JSON structure
        let expected = json!({
            "string": "hello",
            "integer": 42,
            "float": 3.14,
            "boolean": true,
            "array": [1, 2, 3]
        });

        // Compare serialized JSON with expected
        assert_eq!(serialized, expected.to_string());
    }
}
