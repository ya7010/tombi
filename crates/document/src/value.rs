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
pub use table::Table;
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
