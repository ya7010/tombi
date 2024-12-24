use url::Url;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Catalog {
    schemas: Vec<CatalogSchema>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CatalogSchema {
    name: String,
    description: String,
    file_match: Vec<String>,
    url: Url,
}
