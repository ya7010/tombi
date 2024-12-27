use dashmap::DashMap;
use url::Url;

use crate::{
    json_schema::{Catalog, CatalogSchema},
    DocumentSchema,
};

#[derive(Debug, Clone, Default)]
pub struct SchemaStore {
    http_client: reqwest::Client,
    schemas: DashMap<Url, DocumentSchema>,
    catalogs: Vec<CatalogSchema>,
}

impl SchemaStore {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            ..Default::default()
        }
    }

    pub async fn load_catalog(&mut self, catalog_url: &url::Url) {
        if let Ok(response) = self.http_client.get(catalog_url.as_str()).send().await {
            if let Ok(catalog) = response.json::<Catalog>().await {
                for schema in catalog.schemas {
                    if schema
                        .file_match
                        .iter()
                        .any(|pattern| pattern.ends_with(".toml"))
                    {
                        self.catalogs.push(schema);
                    }
                }
            }
        } else {
            tracing::warn!("failed to fetch catalog: {}", catalog_url);
        }
    }

    pub fn add_schema(&mut self, url: Url, schema: DocumentSchema) {
        self.schemas.insert(url, schema);
    }

    pub fn get_schema(&self, url: &Url) -> Option<DocumentSchema> {
        self.schemas.get(url).map(|schema| schema.clone())
    }
}
