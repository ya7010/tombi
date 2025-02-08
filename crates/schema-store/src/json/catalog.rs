use url::Url;

pub const DEFAULT_CATALOG_URL: &str = "https://www.schemastore.org/api/json/catalog.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Catalog {
    pub schemas: Vec<JsonSchema>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchema {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub file_match: Vec<String>,
    pub url: Url,
}
