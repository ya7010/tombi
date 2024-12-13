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
pub enum Value {
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
                .replace('_', "")
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
