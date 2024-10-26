mod array;
mod boolean;
mod date;
mod date_time;
mod float;
mod integer;
mod string;
mod table;
mod time;

pub use array::Array;
pub use boolean::Boolean;
pub use date::Date;
pub use date_time::DateTime;
pub use float::Float;
pub use integer::Integer;
pub use string::String;
pub use table::{Table, TableKind};
pub use time::Time;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String(String),
    DateTime(DateTime),
    Date(Date),
    Time(Time),
    Array(Array),
    Table(Table),
}

impl Value {
    pub fn new(source: &str, value: ast::Value) -> Self {
        match value {
            ast::Value::Boolean(boolean) => Self::Boolean(Boolean::new(source, boolean)),
            ast::Value::IntegerBin(integer) => {
                Self::Integer(Integer::new_integer_bin(source, integer))
            }
            ast::Value::IntegerOct(integer) => {
                Self::Integer(Integer::new_integer_oct(source, integer))
            }
            ast::Value::IntegerDec(integer) => {
                Self::Integer(Integer::new_integer_dec(source, integer))
            }
            ast::Value::IntegerHex(integer) => {
                Self::Integer(Integer::new_integer_hex(source, integer))
            }
            ast::Value::Float(float) => Self::Float(Float::new(source, float)),
            ast::Value::BasicString(string) => {
                Self::String(String::new_basic_string(source, string))
            }
            ast::Value::LiteralString(string) => {
                Self::String(String::new_literal_string(source, string))
            }
            ast::Value::MultiLineBasicString(string) => {
                Self::String(String::new_multi_line_basic_string(source, string))
            }
            ast::Value::MultiLineLiteralString(string) => {
                Self::String(String::new_multi_line_literal_string(source, string))
            }
            ast::Value::OffsetDateTime(date_time) => {
                Self::DateTime(DateTime::new_offset_date_time(source, date_time))
            }
            ast::Value::LocalDateTime(date_time) => {
                Self::DateTime(DateTime::new_local_date_time(source, date_time))
            }
            ast::Value::LocalDate(date) => Self::Date(Date::new_local_date(source, date)),
            ast::Value::LocalTime(time) => Self::Time(Time::new_local_time(source, time)),
            ast::Value::Array(_) => Self::Array(Array::new_array()),
            ast::Value::InlineTable(_) => Self::Table(Table::new(TableKind::InlineTable)),
        }
    }

    pub fn range(&self) -> crate::Range {
        match self {
            Self::Boolean(boolean) => boolean.range(),
            Self::Integer(integer) => integer.range(),
            Self::Float(float) => float.range(),
            Self::String(string) => string.range(),
            Self::DateTime(date_time) => date_time.range(),
            Self::Date(date) => date.range(),
            Self::Time(time) => time.range(),
            Self::Array(array) => array.range(),
            Self::Table(table) => table.range(),
        }
    }
}
