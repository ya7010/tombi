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
use string::StringKind;
pub use table::{Table, TableKind};

use crate::IntoDocument;

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

impl IntoDocument<Value> for document_tree::Value {
    fn into_document(self, toml_version: crate::TomlVersion) -> Value {
        match self {
            document_tree::Value::Boolean(value) => Value::Boolean(value.into()),
            document_tree::Value::Integer(value) => Value::Integer(value.into()),
            document_tree::Value::Float(value) => Value::Float(value.into()),
            document_tree::Value::String(value) => Value::String(value.into()),
            document_tree::Value::OffsetDateTime(value) => Value::OffsetDateTime(value.into()),
            document_tree::Value::LocalDateTime(value) => Value::LocalDateTime(value.into()),
            document_tree::Value::LocalDate(value) => Value::LocalDate(value.into()),
            document_tree::Value::LocalTime(value) => Value::LocalTime(value.into()),
            document_tree::Value::Array(value) => Value::Array(value.into_document(toml_version)),
            document_tree::Value::Table(value) => Value::Table(value.into_document(toml_version)),
            document_tree::Value::Incomplete { .. } => {
                unreachable!("Incomplete value should not be converted to document")
            }
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

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid Value")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Value, E> {
                Ok(Value::Boolean(Boolean::new(v)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Value, E> {
                Ok(Value::Integer(Integer::new(v)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Integer(Integer::new(v as i64)))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Value, E> {
                Ok(Value::Float(Float::new(v)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::String(String::new(
                    StringKind::BasicString,
                    v.to_string(),
                )))
            }

            fn visit_string<E>(self, v: std::string::String) -> Result<Value, E> {
                Ok(Value::String(String::new(
                    StringKind::BasicString,
                    v.to_string(),
                )))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut vec = Array::new(ArrayKind::ArrayOfTables);
                while let Some(elem) = seq.next_element()? {
                    vec.push(elem);
                }
                Ok(Value::Array(vec))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut index_map = Table::new(TableKind::Table);
                while let Some((key, value)) = map.next_entry()? {
                    index_map.insert(key, value);
                }
                Ok(Value::Table(index_map))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}
