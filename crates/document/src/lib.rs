mod key;
mod value;

use std::ops::Deref;

use itertools::Itertools;
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

impl Document {
    /// Convert the document to a TOML string representation
    pub fn to_string(&self) -> std::string::String {
        let mut result = std::string::String::new();
        self.0.to_toml_string(&mut result, &[]);
        result.trim().to_string() + "\n"
    }
}

trait ToTomlString {
    fn to_toml_string(&self, result: &mut std::string::String, parent_keys: &[&crate::Key]);
}

impl ToTomlString for (&Key, &Value) {
    fn to_toml_string(&self, result: &mut std::string::String, parent_keys: &[&crate::Key]) {
        let (key, value) = *self;
        match value {
            Value::Table(table) if table.kind() == TableKind::KeyValue => {
                table.to_toml_string(
                    result,
                    &parent_keys
                        .iter()
                        .chain(&[key])
                        .map(|key| *key)
                        .collect_vec(),
                );
            }
            _ => {
                result.push_str(&format!(
                    "{} = ",
                    parent_keys.iter().chain(&[key]).map(|key| *key).join(".")
                ));
                value.to_toml_string(result, &[]);
            }
        }
        result.push('\n');
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
    use test_lib::toml_text_assert_eq;

    use super::*;

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

        // Test to_string method
        let toml_string = document.to_string();
        let expected = r#"
string = "hello"
integer = 42
float = 3.14
boolean = true
array = [1, 2, 3]
"#;
        toml_text_assert_eq!(toml_string, expected);
    }

    #[test]
    fn test_array_of_tables_serialization() {
        // Create a test document with array of tables
        let mut root_table = Table::new(TableKind::Table);

        // Create array of tables
        let mut array_of_tables = Array::new(ArrayKind::ArrayOfTable);

        // First table in array
        let mut table1 = Table::new(TableKind::Table);
        table1.insert(
            Key::new(KeyKind::BareKey, "name".to_string()),
            Value::String(String::new(StringKind::BasicString, "apple".to_string())),
        );
        table1.insert(
            Key::new(KeyKind::BareKey, "color".to_string()),
            Value::String(String::new(StringKind::BasicString, "red".to_string())),
        );
        array_of_tables.push(Value::Table(table1));

        // Second table in array
        let mut table2 = Table::new(TableKind::Table);
        table2.insert(
            Key::new(KeyKind::BareKey, "name".to_string()),
            Value::String(String::new(StringKind::BasicString, "banana".to_string())),
        );
        table2.insert(
            Key::new(KeyKind::BareKey, "color".to_string()),
            Value::String(String::new(StringKind::BasicString, "yellow".to_string())),
        );
        array_of_tables.push(Value::Table(table2));

        // Add array of tables to root table
        root_table.insert(
            Key::new(KeyKind::BareKey, "fruits".to_string()),
            Value::Array(array_of_tables),
        );

        // Create document
        let document = Document(root_table);

        // Test to_string method
        let toml_string = document.to_string();
        let expected = r#"
[[fruits]]
name = "apple"
color = "red"
[[fruits]]
name = "banana"
color = "yellow"
"#;
        toml_text_assert_eq!(toml_string, expected);
    }

    #[test]
    fn test_nested_tables_serialization() {
        // Create a test document with nested tables
        let mut root_table = Table::new(TableKind::Table);

        // Create nested table
        let mut nested_table = Table::new(TableKind::Table);
        nested_table.insert(
            Key::new(KeyKind::BareKey, "name".to_string()),
            Value::String(String::new(StringKind::BasicString, "John".to_string())),
        );
        nested_table.insert(
            Key::new(KeyKind::BareKey, "age".to_string()),
            Value::Integer(Integer::new(30)),
        );

        // Add nested table to root table
        root_table.insert(
            Key::new(KeyKind::BareKey, "person".to_string()),
            Value::Table(nested_table),
        );

        // Create document
        let document = Document(root_table);

        // Test to_string method
        let toml_string = document.to_string();
        let expected = r#"
[person]
name = "John"
age = 30
"#;
        toml_text_assert_eq!(toml_string, expected);
    }

    #[test]
    fn test_complex_nested_structures_serialization() {
        // Create root table
        let mut root_table = Table::new(TableKind::Table);

        // Create nested table structure [aaa.bbb]
        let mut aaa_table = Table::new(TableKind::Table);
        let mut bbb_table = Table::new(TableKind::Table);

        // Add values to [aaa.bbb]
        bbb_table.insert(
            Key::new(KeyKind::BareKey, "ddd".to_string()),
            Value::String(String::new(StringKind::BasicString, "value1".to_string())),
        );

        // Create and add inline table
        let mut inline_table = Table::new(TableKind::InlineTable);
        inline_table.insert(
            Key::new(KeyKind::BareKey, "x".to_string()),
            Value::Integer(Integer::new(1)),
        );
        inline_table.insert(
            Key::new(KeyKind::BareKey, "y".to_string()),
            Value::Integer(Integer::new(2)),
        );
        bbb_table.insert(
            Key::new(KeyKind::BareKey, "inline".to_string()),
            Value::Table(inline_table),
        );

        // Create nested table [aaa.bbb.ccc]
        let mut ccc_table = Table::new(TableKind::Table);
        ccc_table.insert(
            Key::new(KeyKind::BareKey, "value".to_string()),
            Value::String(String::new(
                StringKind::BasicString,
                "deep nested".to_string(),
            )),
        );

        // Create array of tables
        let mut array_of_tables = Array::new(ArrayKind::ArrayOfTable);
        let mut array_table1 = Table::new(TableKind::Table);
        array_table1.insert(
            Key::new(KeyKind::BareKey, "id".to_string()),
            Value::Integer(Integer::new(1)),
        );
        array_of_tables.push(Value::Table(array_table1));

        let mut array_table2 = Table::new(TableKind::Table);
        array_table2.insert(
            Key::new(KeyKind::BareKey, "id".to_string()),
            Value::Integer(Integer::new(2)),
        );
        array_of_tables.push(Value::Table(array_table2));

        // Add array of tables to ccc_table
        ccc_table.insert(
            Key::new(KeyKind::BareKey, "items".to_string()),
            Value::Array(array_of_tables),
        );

        // Add ccc_table to bbb_table
        bbb_table.insert(
            Key::new(KeyKind::BareKey, "ccc".to_string()),
            Value::Table(ccc_table),
        );

        // Add bbb_table to aaa_table
        aaa_table.insert(
            Key::new(KeyKind::BareKey, "bbb".to_string()),
            Value::Table(bbb_table),
        );

        // Add aaa_table to root table
        root_table.insert(
            Key::new(KeyKind::BareKey, "aaa".to_string()),
            Value::Table(aaa_table),
        );

        // Create document
        let document = Document(root_table);

        // Test to_string method
        let toml_string = document.to_string();
        let expected = r#"
[aaa.bbb]
ddd = "value1"
inline = {x = 1, y = 2}
[aaa.bbb.ccc]
value = "deep nested"
[[aaa.bbb.ccc.items]]
id = 1
[[aaa.bbb.ccc.items]]
id = 2
"#;
        toml_text_assert_eq!(toml_string, expected);
    }
}
