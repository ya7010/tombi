mod de;
mod document;
mod ser;

pub use de::{from_document, from_str, parse_str};
pub use ser::{to_document, to_string, to_string_async, Serializer};
use std::fmt;
use thiserror::Error;

pub use document::{
    Array, ArrayKind, Boolean, Document, Float, Integer, IntegerKind, Key, LocalDate,
    LocalDateTime, LocalTime, OffsetDateTime, String, StringKind, Table, TableKind, Value,
};

/// Error that can occur when processing TOML.
#[derive(Debug, Error)]
pub enum Error {
    /// Error occurred while parsing the TOML document.
    #[error("Parser error: {0}")]
    Parser(std::string::String),

    /// Error occurred during document tree construction.
    #[error("Document tree error: {0}")]
    DocumentTree(std::string::String),

    /// Error occurred during serialization.
    #[error("Serialization error: {0}")]
    Serialization(std::string::String),

    /// Error occurred during deserialization.
    #[error("Deserialization error: {0}")]
    Deserialization(std::string::String),
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Serialization(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Deserialization(msg.to_string())
    }
}

/// A specialized `Result` type for serde_tombi operations.
pub type Result<T> = std::result::Result<T, Error>;
