use ast::AstNode;
use config::TomlVersion;
use std::io::Read;
use toml_test::INVALID_MESSAGE;

fn main() -> Result<(), anyhow::Error> {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source)?;

    let value = decode(&source)?;
    println!("{}", serde_json::to_string_pretty(&value).unwrap());

    Ok(())
}

fn decode(source: &str) -> Result<Value, anyhow::Error> {
    let p = parser::parse(source, TomlVersion::default());

    if !p.errors().is_empty() {
        for error in p.errors() {
            eprintln!("{}", error);
        }
        return Err(anyhow::anyhow!(INVALID_MESSAGE));
    }

    let Some(root) = ast::Root::cast(p.into_syntax_node()) else {
        eprintln!("ast root cast failed");
        return Err(anyhow::anyhow!(INVALID_MESSAGE));
    };

    let root = match document_tree::Root::try_from(root) {
        Ok(root) => root,
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
            return Err(anyhow::anyhow!(INVALID_MESSAGE));
        }
    };

    Ok(Value::from(root))
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Type {
    Bool,
    Integer,
    Float,
    String,
    Datetime,
    DatetimeLocal,
    DateLocal,
    TimeLocal,
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
enum Value {
    Literal { r#type: Type, value: String },
    Array(Vec<Value>),
    Table(indexmap::IndexMap<String, Value>),
}

impl From<document_tree::Value> for Value {
    fn from(node: document_tree::Value) -> Self {
        match node {
            document_tree::Value::Boolean(value) => Self::Literal {
                r#type: Type::Bool,
                value: value.node().token().unwrap().text().to_string(),
            },
            document_tree::Value::Integer(value) => Self::Literal {
                r#type: Type::Integer,
                value: match value.kind() {
                    document_tree::IntegerKind::Decimal(node) => node.token(),
                    document_tree::IntegerKind::Hexadecimal(node) => node.token(),
                    document_tree::IntegerKind::Octal(node) => node.token(),
                    document_tree::IntegerKind::Binary(node) => node.token(),
                }
                .unwrap()
                .text()
                .to_string(),
            },
            document_tree::Value::Float(value) => Self::Literal {
                r#type: Type::Float,
                value: value.node().token().unwrap().text().to_string(),
            },
            document_tree::Value::String(value) => Self::Literal {
                r#type: Type::String,
                value: value.raw_string(),
            },
            document_tree::Value::OffsetDateTime(value) => Self::Literal {
                r#type: Type::Datetime,
                value: value.node().token().unwrap().text().to_string(),
            },
            document_tree::Value::LocalDateTime(value) => Self::Literal {
                r#type: Type::DatetimeLocal,
                value: value.node().token().unwrap().text().to_string(),
            },
            document_tree::Value::LocalDate(value) => Self::Literal {
                r#type: Type::DateLocal,
                value: value.node().token().unwrap().text().to_string(),
            },
            document_tree::Value::LocalTime(value) => Self::Literal {
                r#type: Type::TimeLocal,
                value: value.node().token().unwrap().text().to_string(),
            },
            document_tree::Value::Array(value) => {
                Self::Array(value.into_iter().map(Value::from).collect())
            }
            document_tree::Value::Table(value) => Self::Table(
                value
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect(),
            ),
        }
    }
}

impl From<document_tree::Root> for Value {
    fn from(node: document_tree::Root) -> Self {
        Self::Table(
            document_tree::Table::from(node)
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

#[cfg(test)]
macro_rules! test_decode {
    {
        #[test]
        fn $name:ident($source:expr) -> Ok($expected:expr)
    } => {
        #[test]
        fn $name()
        {
        let source = textwrap::dedent($source);
        let value = crate::decode(source.trim()).unwrap();
        pretty_assertions::assert_eq!(
            serde_json::to_string(&value).unwrap(),
            serde_json::to_string(&$expected).unwrap()
        );
    }
    };
}

#[cfg(test)]
mod test {
    use serde_json::json;

    test_decode! {
        #[test]
        fn valid_array_array(
            r#"
            mixed = [[1, 2], ["a", "b"], [1.1, 2.1]]
            "#
        ) -> Ok(json!(
                {
                    "mixed": [
                        [
                            {"type": "integer", "value": "1"},
                            {"type": "integer", "value": "2"}
                        ],
                        [
                            {"type": "string", "value": "a"},
                            {"type": "string", "value": "b"}
                        ],
                        [
                            {"type": "float", "value": "1.1"},
                            {"type": "float", "value": "2.1"}
                        ]
                    ]
                }
            )
        )
    }
}
