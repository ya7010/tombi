#[derive(Debug, Clone)]
pub struct CatalogSchema {
    pub toml_version: Option<config::TomlVersion>,
    pub url: crate::SchemaUrl,
    pub include: Vec<String>,
    pub root_keys: Option<Vec<String>>,
}
