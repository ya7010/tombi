use dashmap::{mapref::one::Ref, DashMap};
use url::Url;

use crate::{
    json_schema::{Catalog, CatalogSchema},
    DocumentSchema,
};

#[derive(Debug, Clone, Default)]
pub struct SchemaStore {
    http_client: reqwest::Client,
    schemas: DashMap<Url, DocumentSchema>,
    catalogs: DashMap<Url, CatalogSchema>,
}

impl SchemaStore {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            ..Default::default()
        }
    }

    pub async fn load_catalog(&self, catalog_url: &url::Url) {
        tracing::debug!("loading schema catalog: {}", catalog_url);

        if let Ok(response) = self.http_client.get(catalog_url.as_str()).send().await {
            if let Ok(catalog) = response.json::<Catalog>().await {
                for schema in catalog.schemas {
                    if schema
                        .file_match
                        .iter()
                        .any(|pattern| pattern.ends_with(".toml"))
                    {
                        self.catalogs.insert(schema.url.clone(), schema);
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

    pub fn get_schema_from_url(&self, url: &Url) -> Option<Ref<'_, Url, DocumentSchema>> {
        self.schemas.get(url)
    }

    pub fn get_schema_from_source(
        &self,
        source_path: &std::path::Path,
    ) -> Option<Ref<'_, Url, DocumentSchema>> {
        for catalog in &self.catalogs {
            if catalog.file_match.iter().any(|pat| {
                glob::Pattern::new(pat)
                    .ok()
                    .map(|glob_pat| glob_pat.matches_path(source_path))
                    .unwrap_or(false)
            }) {
                if let Some(schema) = self.get_schema_from_url(&catalog.url) {
                    return Some(schema);
                }
            }
        }
        None
    }
}
