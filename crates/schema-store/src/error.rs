use url::Url;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("failed to parse catalog: {catalog_url}")]
    CatalogParseFailed { catalog_url: Url },

    #[error("failed to fetch catalog: {catalog_url}")]
    CatalogFetchFailed { catalog_url: Url },

    #[error("unsupported url schema: {url_schema}")]
    UnsupportedUrlSchema { url_schema: String },

    #[error("failed to read schema: \"{schema_path}\"")]
    SchemaFileReadFailed { schema_path: String },

    #[error("failed to parse schema: \"{schema_path}\"")]
    SchemaFileParseFailed { schema_path: String },

    #[error("failed to fetch schema: {schema_url}")]
    SchemaFetchFailed { schema_url: String },
}
