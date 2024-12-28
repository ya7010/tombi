use config::SchemaOptions;
use dashmap::DashMap;
use url::Url;

use crate::{json_schema::JsonCatalog, schema::CatalogSchema, DocumentSchema};

#[derive(Debug, Clone, Default)]
pub struct SchemaStore {
    http_client: reqwest::Client,
    schemas: DashMap<Url, Result<DocumentSchema, crate::Error>>,
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

    pub async fn load_catalog(&self, catalog_url: &url::Url) -> Result<(), crate::Error> {
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
                Ok(())
            } else {
                Err(crate::Error::CatalogParseFailed {
                    catalog_url: catalog_url.clone(),
                })
            }
        } else {
            Err(crate::Error::CatalogFetchFailed {
                catalog_url: catalog_url.clone(),
            })
        }
    }

    pub fn get_schema_from_url<'a>(&'a self, url: &Url) -> Result<DocumentSchema, crate::Error> {
        match self.schemas.get(url) {
            Some(schema) => match schema.value() {
                Ok(schema) => Ok(schema.clone()),
                Err(err) => Err(err.clone()),
            },
            None => {
                let document_schema = match url.scheme() {
                    "file" => {
                        let file = std::fs::File::open(url.path()).map_err(|_| {
                            crate::Error::SchemaFileReadFailed {
                                schema_path: url.path().to_string(),
                            }
                        })?;

                        let data: serde_json::Value =
                            serde_json::from_reader(file).map_err(|_| {
                                crate::Error::SchemaFileParseFailed {
                                    schema_path: url.path().to_string(),
                                }
                            })?;
                        DocumentSchema {
                            title: data
                                .get("title")
                                .map(|obj| obj.as_str().map(|title| title.to_string()))
                                .flatten(),
                            description: data
                                .get("description")
                                .map(|obj| obj.as_str().map(|title| title.to_string()))
                                .flatten(),
                            schema_url: Some(url.to_owned()),
                            ..Default::default()
                        }
                    }
                    _ => {
                        return Err(crate::Error::UnsupportedUrlSchema {
                            url_schema: url.scheme().to_string(),
                        })
                    }
                };

                self.schemas
                    .insert(url.to_owned(), Ok(document_schema.clone()));

                Ok(document_schema)
            }
        }
    }

    pub fn get_schema_from_source(&self, source_path: &std::path::Path) -> Option<DocumentSchema> {
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
                if let Ok(schema) = self.get_schema_from_url(catalog_url) {
                    return Some(schema);
                }
            }
        }
        None
    }
}
