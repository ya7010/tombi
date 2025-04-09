mod error;

use ast::AstNode;
use document::IntoDocument;
use document_tree::IntoDocumentTreeAndErrors;
pub use error::Error;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use toml_version::TomlVersion;

/// Deserialize a TOML string into a Rust data structure.
///
/// # Note
///
/// This function is not yet implemented and will return an error.
/// The example below shows the expected usage once implemented.
///
/// # Examples
///
/// ```no_run
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     ip: String,
///     port: u16,
///     keys: Vec<String>,
/// }
///
/// let toml = r#"
/// ip = "127.0.0.1"
/// port = 8080
/// keys = ["key1", "key2"]
/// "#;
///
/// let config: Config = serde_tombi::from_str(toml).unwrap();
/// ```
pub fn from_str<T>(s: &str) -> Result<T, crate::de::Error>
where
    T: DeserializeOwned,
{
    let document = parse_str(s)?;
    from_document(document)
}

pub fn from_document<T>(document: document::Document) -> Result<T, crate::de::Error>
where
    T: DeserializeOwned,
{
    Ok(T::deserialize(&document)?)
}

/// Parse a TOML string into a Document.
pub fn parse_str(s: &str) -> Result<document::Document, crate::de::Error> {
    // Parse the source string using the parser
    let parsed = parser::parse(s);

    let errors = parsed.errors(TomlVersion::default()).collect_vec();
    // Check if there are any parsing errors
    if !errors.is_empty() {
        return Err(crate::de::Error::Parser(
            parsed.into_errors(TomlVersion::default()).collect_vec(),
        ));
    }

    // Cast the parsed result to an AST Root node
    let root = ast::Root::cast(parsed.into_syntax_node()).expect("AST Root must be present");

    // Convert the AST to a document tree
    let (document_tree, errors) = root
        .into_document_tree_and_errors(TomlVersion::default())
        .into();

    // Check for errors during document tree construction
    if !errors.is_empty() {
        return Err(crate::de::Error::DocumentTree(errors));
    }

    // Convert to a Document
    Ok(document_tree.into_document(TomlVersion::default()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};
    use indexmap::{indexmap, IndexMap};
    use serde::Deserialize;

    #[test]
    fn test_deserialize_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            int: i32,
            float: f64,
            string: String,
            bool: bool,
            opt: Option<String>,
        }

        let toml = r#"
int = 42
float = 3.141592653589793
string = "hello"
bool = true
opt = "optional"
"#;

        let expected = Test {
            int: 42,
            float: std::f64::consts::PI,
            string: "hello".to_string(),
            bool: true,
            opt: Some("optional".to_string()),
        };

        let result: Test = from_str(toml).expect("TOML deserialization failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize_nested_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Nested {
            value: String,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            nested: Nested,
            simple_value: i32,
        }

        let toml = r#"
simple_value = 42

[nested]
value = "nested value"
"#;

        let expected = Test {
            nested: Nested {
                value: "nested value".to_string(),
            },
            simple_value: 42,
        };

        let result: Test = from_str(toml).expect("TOML deserialization failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize_array() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct SimpleArrayTest {
            values: Vec<i32>,
        }

        let toml = r#"values = [1, 2, 3]"#;

        let expected = SimpleArrayTest {
            values: vec![1, 2, 3],
        };

        let result: SimpleArrayTest = from_str(toml).expect("TOML deserialization failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize_map() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct MapTest {
            string_map: IndexMap<String, String>,
            int_map: IndexMap<String, i32>,
        }

        let toml = r#"
[string_map]
key1 = "value1"
key2 = "value2"

[int_map]
one = 1
two = 2
three = 3
"#;

        let expected = MapTest {
            string_map: indexmap! {
                "key1".to_string() => "value1".to_string(),
                "key2".to_string() => "value2".to_string(),
            },
            int_map: indexmap! {
                "one".to_string() => 1,
                "two".to_string() => 2,
                "three".to_string() => 3,
            },
        };

        let result: MapTest = from_str(toml).expect("TOML deserialization failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize_enum() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum SimpleEnum {
            Variant1,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct EnumTest {
            enum_value: SimpleEnum,
        }

        let toml = r#"enum_value = "Variant1""#;

        let expected = EnumTest {
            enum_value: SimpleEnum::Variant1,
        };

        let result: EnumTest = from_str(toml).expect("TOML deserialization failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize_datetime() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct DateTimeTest {
            created_at: DateTime<Utc>,
            updated_at: DateTime<Utc>,
        }

        let toml = r#"
created_at = "2023-05-15T10:30:00Z"
updated_at = "2023-07-20T14:45:30Z"
"#;

        let expected = DateTimeTest {
            created_at: Utc.with_ymd_and_hms(2023, 5, 15, 10, 30, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2023, 7, 20, 14, 45, 30).unwrap(),
        };

        let result: DateTimeTest = from_str(toml).expect("TOML deserialization failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize_option() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct OptionTest {
            some: Option<String>,
            none: Option<String>,
        }

        let toml = r#"some = "optional""#;

        let expected = OptionTest {
            some: Some("optional".to_string()),
            none: None,
        };

        let result: OptionTest = from_str(toml).expect("TOML deserialization failed");
        assert_eq!(result, expected);
    }
}
