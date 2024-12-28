use config::SchemaOptions;
use dashmap::{mapref::one::Ref, DashMap};
use url::Url;

use crate::{json_schema::JsonCatalog, schema::CatalogSchema, DocumentSchema};

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

    pub fn load_config_schema(
        &self,
        config_path: Option<std::path::PathBuf>,
        schemas: Vec<SchemaOptions>,
    ) {
        let config_path = match config_path {
            Some(config_path) => config_path,
            None => match std::env::current_dir() {
                Ok(current_dir) => current_dir,
                Err(_) => return,
            },
        };

        let Some(config_dir) = config_path.parent() else {
            return;
        };
        for schema in schemas {
            let Ok(url) = Url::parse(&format!(
                "file://{}",
                config_dir.join(schema.path).to_string_lossy()
            )) else {
                continue;
            };
            self.catalogs.insert(
                url,
                CatalogSchema {
                    include: schema.include.unwrap_or_default(),
                },
            );
        }
    }

    pub async fn load_catalog(&self, catalog_url: &url::Url) {
        tracing::debug!("loading schema catalog: {}", catalog_url);

        if let Ok(response) = self.http_client.get(catalog_url.as_str()).send().await {
            if let Ok(catalog) = response.json::<JsonCatalog>().await {
                for schema in catalog.schemas {
                    if schema
                        .file_match
                        .iter()
                        .any(|pattern| pattern.ends_with(".toml"))
                    {
                        self.catalogs.insert(
                            schema.url,
                            CatalogSchema {
                                include: schema.file_match,
                            },
                        );
                    }
                }
            } else {
                tracing::warn!("failed to parse catalog: {}", catalog_url);
            }
        } else {
            tracing::warn!("failed to fetch catalog: {}", catalog_url);
        }
    }

    pub fn add_schema(&mut self, url: Url, schema: DocumentSchema) {
        self.schemas.insert(url, schema);
    }

    pub fn get_schema_from_url<'a>(&'a self, url: &Url) -> Option<Ref<'a, Url, DocumentSchema>> {
        self.schemas.get(url)
    }

    pub fn get_schema_from_source(
        &self,
        source_path: &std::path::Path,
    ) -> Option<Ref<'_, Url, DocumentSchema>> {
        for catalog in &self.catalogs {
            let catalog_url = catalog.key();
            if catalog.include.iter().any(|pat| {
                let pattern = if !pat.contains("*") {
                    format!("**/{}", pat)
                } else {
                    pat.to_string()
                };
                glob::Pattern::new(&pattern)
                    .ok()
                    .map(|glob_pat| glob_pat.matches_path(source_path))
                    .unwrap_or(false)
            }) {
                if let Some(schema) = self.get_schema_from_url(catalog_url) {
                    return Some(schema);
                }
            }
        }
        None
    }
}
