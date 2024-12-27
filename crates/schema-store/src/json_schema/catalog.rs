use url::Url;

pub const DEFAULT_CATALOG_URL: &str = "https://www.schemastore.org/api/json/catalog.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Catalog {
    pub schemas: Vec<CatalogSchema>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CatalogSchema {
    pub name: String,
    pub description: String,
    pub file_match: Vec<String>,
    pub url: Url,
}
