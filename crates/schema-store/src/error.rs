use url::Url;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("failed to parse catalog: {catalog_url}")]
    CatalogParseFailed { catalog_url: Url },

    #[error("failed to fetch catalog: {catalog_url}")]
    CatalogFetchFailed { catalog_url: Url },

    #[error("unsupported schema url: {schema_url}")]
    UrlSchemaUnsupported { schema_url: Url },

    #[error("failed to parse schema url: {schema_url}")]
    UrlSchemaParseFailed { schema_url: Url },

    #[error("failed to read schema: \"{schema_path}\"")]
    SchemaFileReadFailed { schema_path: String },

    #[error("failed to parse schema: \"{schema_url}\"")]
    SchemaFileParseFailed { schema_url: Url },

    #[error("failed to fetch schema: {schema_url}")]
    SchemaFetchFailed { schema_url: String },
}
