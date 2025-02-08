#[derive(Debug, Clone)]
pub struct CatalogSchema {
    pub url: crate::SchemaUrl,
    pub include: Vec<String>,
}
