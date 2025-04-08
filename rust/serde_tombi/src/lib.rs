//! Provides serialization functionality for converting Rust types to TOML.
//!
//! # Examples
//!
//! ## Basic usage
//!
//! ```rust
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct Config {
//!     ip: String,
//!     port: u16,
//! }
//!
//! let config = Config {
//!     ip: "127.0.0.1".to_string(),
//!     port: 8080,
//! };
//!
//! // Simple serialization
//! let toml = serde_tombi::to_string(&config).unwrap();
//! ```
//!
//! ## Using TypedBuilder pattern
//!
//! ```rust
//! use serde::Serialize;
//! use serde_tombi::Serializer;
//!
//! #[derive(Serialize)]
//! struct Config {
//!     ip: String,
//!     port: u16,
//! }
//!
//! let config = Config {
//!     ip: "127.0.0.1".to_string(),
//!     port: 8080,
//! };
//!
//! // Using either the builder pattern or direct construction
//! // Builder pattern:
//! let serializer = Serializer::builder()
//!     .schema_store(&schema_store::SchemaStore::default())
//!     .build();
//!
//! // Or direct construction:
//! let serializer = Serializer::new();
//!
//! let toml = serializer.to_string(&config).unwrap();
//! ```
//!
mod de;
mod document;
mod ser;

pub use de::{from_document, from_str, parse_str};
pub use document::{
    Array, ArrayKind, Boolean, Document, Float, Integer, IntegerKind, Key, LocalDate,
    LocalDateTime, LocalTime, OffsetDateTime, String, StringKind, Table, TableKind, Value,
};

pub use ser::{to_document, to_string, to_string_async, Serializer};
use std::fmt;
use thiserror::Error;

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
