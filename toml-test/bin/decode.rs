use ast::AstNode;
use config::TomlVersion;
use std::io::Read;
use toml_test::INVALID_MESSAGE;

fn main() -> Result<(), anyhow::Error> {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source)?;

    let p = parser::parse(&source, TomlVersion::default());

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

    let document = Value::from(root);

    println!("{}", serde_json::to_string_pretty(&document).unwrap());

    Ok(())
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
    Boolean {
        r#type: Type,
        value: bool,
    },
    Integer {
        r#type: Type,
        value: i64,
    },
    Float {
        r#type: Type,
        value: f64,
    },
    String {
        r#type: Type,
        value: String,
    },
    OffsetDateTime {
        r#type: Type,
        value: chrono::DateTime<chrono::FixedOffset>,
    },
    LocalDateTime {
        r#type: Type,
        value: chrono::NaiveDateTime,
    },
    LocalDate {
        r#type: Type,
        value: chrono::NaiveDate,
    },
    LocalTime {
        r#type: Type,
        value: chrono::NaiveTime,
    },
    Array(Vec<Value>),
    Table(indexmap::IndexMap<String, Value>),
}

impl From<document_tree::Value> for Value {
    fn from(node: document_tree::Value) -> Self {
        match node {
            document_tree::Value::Boolean(value) => Self::Boolean {
                r#type: Type::Bool,
                value: value.value(),
            },
            document_tree::Value::Integer(value) => Self::Integer {
                r#type: Type::Integer,
                value: value.value(),
            },
            document_tree::Value::Float(value) => Self::Float {
                r#type: Type::Float,
                value: value.value(),
            },
            document_tree::Value::String(value) => Self::String {
                r#type: Type::String,
                value: value.raw_string(),
            },
            document_tree::Value::OffsetDateTime(value) => Self::OffsetDateTime {
                r#type: Type::Datetime,
                value: *value.value(),
            },
            document_tree::Value::LocalDateTime(value) => Self::LocalDateTime {
                r#type: Type::DatetimeLocal,
                value: *value.value(),
            },
            document_tree::Value::LocalDate(value) => Self::LocalDate {
                r#type: Type::DateLocal,
                value: *value.value(),
            },
            document_tree::Value::LocalTime(value) => Self::LocalTime {
                r#type: Type::TimeLocal,
                value: *value.value(),
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
