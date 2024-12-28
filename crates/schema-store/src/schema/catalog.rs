#[derive(Debug, Clone)]
pub struct CatalogSchema {
    pub url: url::Url,
    pub include: Vec<String>,
}
