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

    #[error("failed to parse catalog: {catalog_url}")]
    CatalogUrlParseFailed { catalog_url: CatalogUrl },

    #[error("failed to fetch catalog: {catalog_url}")]
    CatalogUrlFetchFailed { catalog_url: CatalogUrl },

    #[error("unsupported schema url: {schema_url}")]
    SchemaUrlUnsupported { schema_url: SchemaUrl },

    #[error("failed to parse schema url: {schema_url}")]
    SchemaUrlParseFailed { schema_url: SchemaUrl },

    #[error("failed to read schema: \"{schema_path}\"")]
    SchemaFileReadFailed { schema_path: PathBuf },

    #[error("failed to parse schema: \"{schema_url}\"")]
    SchemaFileParseFailed { schema_url: SchemaUrl },

    #[error("failed to fetch schema: {schema_url}")]
    SchemaFetchFailed { schema_url: SchemaUrl },

    #[error("unsupported source url: {source_url}")]
    SourceUrlUnsupported { source_url: url::Url },

    #[error("failed to parse source url: {source_url}")]
    SourceUrlParseFailed { source_url: url::Url },
}
