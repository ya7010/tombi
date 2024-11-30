use super::{Array, Table};

#[derive(Debug)]
pub enum Value {
    Boolean { range: text::Range },
    Integer { range: text::Range },
    Float { range: text::Range },
    String { range: text::Range },
    OffsetDateTime { range: text::Range },
    LocalDateTime { range: text::Range },
    LocalDate { range: text::Range },
    LocalTime { range: text::Range },
    Array(Array),
    Table(Table),
}

impl Value {
    pub fn range(&self) -> text::Range {
        match self {
            Self::Boolean { range } => range.clone(),
            Self::Integer { range } => range.clone(),
            Self::Float { range } => range.clone(),
            Self::String { range } => range.clone(),
            Self::OffsetDateTime { range } => range.clone(),
            Self::LocalDateTime { range } => range.clone(),
            Self::LocalDate { range } => range.clone(),
            Self::LocalTime { range } => range.clone(),
            Self::Array(array) => array.range(),
            Self::Table(table) => table.range(),
        }
    }
}

impl From<ast::Value> for Value {
    fn from(node: ast::Value) -> Self {
        match node {
            ast::Value::Boolean(value) => Value::Boolean {
                range: value.range(),
            },
            ast::Value::IntegerBin(value) => Value::Integer {
                range: value.range(),
            },
            ast::Value::IntegerOct(value) => Value::Integer {
                range: value.range(),
            },
            ast::Value::IntegerDec(value) => Value::Integer {
                range: value.range(),
            },
            ast::Value::IntegerHex(value) => Value::Integer {
                range: value.range(),
            },
            ast::Value::Float(value) => Value::Float {
                range: value.range(),
            },
            ast::Value::BasicString(value) => Value::String {
                range: value.range(),
            },
            ast::Value::LiteralString(value) => Value::String {
                range: value.range(),
            },
            ast::Value::MultiLineBasicString(value) => Value::String {
                range: value.range(),
            },
            ast::Value::MultiLineLiteralString(value) => Value::String {
                range: value.range(),
            },
            ast::Value::OffsetDateTime(value) => Value::OffsetDateTime {
                range: value.range(),
            },
            ast::Value::LocalDateTime(value) => Value::LocalDateTime {
                range: value.range(),
            },
            ast::Value::LocalDate(value) => Value::LocalDate {
                range: value.range(),
            },
            ast::Value::LocalTime(value) => Value::LocalTime {
                range: value.range(),
            },
            ast::Value::Array(value) => Value::Array(value.into()),
            ast::Value::InlineTable(value) => Value::Table(value.into()),
        }
    }
}
