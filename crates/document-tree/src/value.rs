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
pub use string::{String, StringKind};
pub use table::{Table, TableKind};

use crate::TryIntoDocumentTree;

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

impl Value {
    #[inline]
    pub fn range(&self) -> text::Range {
        match self {
            Value::Boolean(value) => value.range(),
            Value::Integer(value) => value.range(),
            Value::Float(value) => value.range(),
            Value::String(value) => value.range(),
            Value::OffsetDateTime(value) => value.range(),
            Value::LocalDateTime(value) => value.range(),
            Value::LocalDate(value) => value.range(),
            Value::LocalTime(value) => value.range(),
            Value::Array(value) => value.range(),
            Value::Table(value) => value.range(),
        }
    }

    #[inline]
    pub fn symbol_range(&self) -> text::Range {
        match self {
            Value::Boolean(value) => value.symbol_range(),
            Value::Integer(value) => value.symbol_range(),
            Value::Float(value) => value.symbol_range(),
            Value::String(value) => value.symbol_range(),
            Value::OffsetDateTime(value) => value.symbol_range(),
            Value::LocalDateTime(value) => value.symbol_range(),
            Value::LocalDate(value) => value.symbol_range(),
            Value::LocalTime(value) => value.symbol_range(),
            Value::Array(value) => value.symbol_range(),
            Value::Table(value) => value.symbol_range(),
        }
    }
}

impl TryIntoDocumentTree<Value> for ast::Value {
    fn try_into_document_tree(
        self,
        toml_version: config::TomlVersion,
    ) -> Result<Value, Vec<crate::Error>> {
        match self {
            ast::Value::BasicString(string) => string.try_into().map(Value::String),
            ast::Value::LiteralString(string) => string.try_into().map(Value::String),
            ast::Value::MultiLineBasicString(string) => string.try_into().map(Value::String),
            ast::Value::MultiLineLiteralString(string) => string.try_into().map(Value::String),
            ast::Value::IntegerBin(integer) => integer.try_into().map(Value::Integer),
            ast::Value::IntegerOct(integer) => integer.try_into().map(Value::Integer),
            ast::Value::IntegerDec(integer) => integer.try_into().map(Value::Integer),
            ast::Value::IntegerHex(integer) => integer.try_into().map(Value::Integer),
            ast::Value::Float(float) => float.try_into().map(Value::Float),
            ast::Value::Boolean(boolean) => boolean.try_into().map(Value::Boolean),
            ast::Value::OffsetDateTime(dt) => dt
                .try_into_document_tree(toml_version)
                .map(Value::OffsetDateTime),
            ast::Value::LocalDateTime(dt) => dt
                .try_into_document_tree(toml_version)
                .map(Value::LocalDateTime),
            ast::Value::LocalDate(date) => date.try_into().map(Value::LocalDate),
            ast::Value::LocalTime(time) => time.try_into().map(Value::LocalTime),
            ast::Value::Array(array) => {
                array.try_into_document_tree(toml_version).map(Value::Array)
            }
            ast::Value::InlineTable(inline_table) => inline_table
                .try_into_document_tree(toml_version)
                .map(Value::Table),
        }
    }
}
