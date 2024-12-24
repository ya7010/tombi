use dashmap::DashMap;
use url::Url;

use crate::{json_schema::Catalog, DocumentSchema};

#[derive(Debug, Clone, Default)]
pub struct SchemaStore {
    http_client: reqwest::Client,
    schemas: DashMap<Url, DocumentSchema>,
    catalogs: Vec<Catalog>,
}

impl SchemaStore {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            ..Default::default()
        }
    }

    pub async fn from_catalog_urls(catalog_urls: &[url::Url]) -> Self {
        let mut store = Self::new();
        for catalog_url in catalog_urls {
            if let Ok(response) = store.http_client.get(catalog_url.as_str()).send().await {
                if let Ok(catalog) = response.json::<Catalog>().await {
                    store.catalogs.push(catalog);
                }
            } else {
                tracing::warn!("Failed to fetch catalog: {}", catalog_url);
            }
        }
        store
    }

    pub fn add_schema(&mut self, url: Url, schema: DocumentSchema) {
        self.schemas.insert(url, schema);
    }

    pub fn get_schema(&self, url: &Url) -> Option<DocumentSchema> {
        match self.schemas.get(url) {
            Some(schema) => Some(schema.clone()),
            None => None,
        }
    }
}
