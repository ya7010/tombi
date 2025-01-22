mod error;
mod key;
mod root;
pub mod support;
mod value;
mod value_type;

pub use error::Error;
pub use key::{Key, KeyKind};
pub use root::Root;
use toml_version::TomlVersion;
pub use value::{
    Array, ArrayKind, Boolean, Float, Integer, IntegerKind, LocalDate, LocalDateTime, LocalTime,
    OffsetDateTime, String, StringKind, Table, TableKind, Value,
};
pub use value_type::ValueType;

pub trait ValueImpl {
    fn value_type(&self) -> ValueType;

    fn range(&self) -> text::Range;
}

pub trait TryIntoDocumentTree<T> {
    fn try_into_document_tree(self, toml_version: TomlVersion) -> Result<T, Vec<crate::Error>>;
}
