use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse catalog: {catalog_url}")]
    CatalogParseFailed { catalog_url: Url },

    #[error("failed to fetch catalog: {catalog_url}")]
    CatalogFetchFailed { catalog_url: Url },
}
