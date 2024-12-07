mod array;
mod boolean;
mod date_time;
mod float;
mod integer;
mod string;
mod table;

pub use array::{Array, ArrayKind};
pub use boolean::Boolean;
pub use date_time::{LocalDate, LocalDateTime, LocalTime, OffsetDateTime};
pub use float::Float;
pub use integer::{Integer, IntegerKind};
pub use string::String;
pub use table::{Table, TableKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String(String),
    OffsetDateTime(OffsetDateTime),
    LocalDateTime(LocalDateTime),
    LocalDate(LocalDate),
    LocalTime(LocalTime),
    Array(Array),
    Table(Table),
}

impl From<document_tree::Value> for Value {
    fn from(node: document_tree::Value) -> Self {
        match node {
            document_tree::Value::Boolean(value) => Self::Boolean(value.into()),
            document_tree::Value::Integer(value) => Self::Integer(value.into()),
            document_tree::Value::Float(value) => Self::Float(value.into()),
            document_tree::Value::String(value) => Self::String(value.into()),
            document_tree::Value::OffsetDateTime(value) => Self::OffsetDateTime(value.into()),
            document_tree::Value::LocalDateTime(value) => Self::LocalDateTime(value.into()),
            document_tree::Value::LocalDate(value) => Self::LocalDate(value.into()),
            document_tree::Value::LocalTime(value) => Self::LocalTime(value.into()),
            document_tree::Value::Array(value) => Self::Array(value.into()),
            document_tree::Value::Table(value) => Self::Table(value.into()),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Boolean(value) => value.serialize(serializer),
            Value::Integer(value) => value.serialize(serializer),
            Value::Float(value) => value.serialize(serializer),
            Value::String(value) => value.serialize(serializer),
            Value::OffsetDateTime(value) => value.serialize(serializer),
            Value::LocalDateTime(value) => value.serialize(serializer),
            Value::LocalDate(value) => value.serialize(serializer),
            Value::LocalTime(value) => value.serialize(serializer),
            Value::Array(value) => value.serialize(serializer),
            Value::Table(value) => value.serialize(serializer),
        }
    }
}
