use std::path::PathBuf;

use crate::{json::CatalogUrl, SchemaUrl};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("failed to lock document: {schema_url}")]
    DocumentLockError { schema_url: SchemaUrl },

    #[error("failed to lock reference: {ref_string}")]
    ReferenceLockError { ref_string: String },

    #[error("failed to lock schema")]
    SchemaLockError,

    #[error("definition ref not found: {definition_ref}")]
    DefinitionNotFound { definition_ref: String },

    #[error("failed to fetch catalog: {catalog_url}")]
    CatalogUrlFetchFailed { catalog_url: CatalogUrl },

    #[error("unsupported schema url: {schema_url}")]
    UnsupportedSchemaUrl { schema_url: SchemaUrl },

    #[error("invalid schema url: {schema_url}")]
    InvalidSchemaUrl { schema_url: String },

    #[error("failed to read schema: \"{schema_path}\"")]
    SchemaFileReadFailed { schema_path: PathBuf },

    #[error("failed to parse schema: {schema_url}, reason: {reason}")]
    SchemaFileParseFailed {
        schema_url: SchemaUrl,
        reason: String,
    },

    #[error("failed to fetch schema: {schema_url}, reason: {reason}")]
    SchemaFetchFailed {
        schema_url: SchemaUrl,
        reason: String,
    },

    #[error("unsupported source url: {source_url}")]
    SourceUrlUnsupported { source_url: url::Url },

    #[error("invalid source url: {source_url}")]
    SourceUrlParseFailed { source_url: url::Url },

    #[error("invalid file path: {url}")]
    InvalidFilePath { url: url::Url },

    #[error("invalid json format: {url}, reason: {reason}")]
    InvalidJsonFormat { url: url::Url, reason: String },

    #[error("invalid json schema reference: {reference}")]
    InvalidJsonSchemaReference { reference: String },

    #[error("unsupported reference: {reference}")]
    UnsupportedReference { reference: String },
}
