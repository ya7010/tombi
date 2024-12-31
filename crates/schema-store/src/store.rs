use config::SchemaInfo;
use dashmap::DashMap;
use itertools::Either;
use std::sync::{Arc, RwLock};
use url::Url;

use crate::{
    json_schema::JsonCatalog, schema::CatalogSchema, Accessor, DocumentSchema, ObjectSchema,
};

#[derive(Debug, Clone, Default)]
pub struct SchemaStore {
    http_client: reqwest::Client,
    schemas: DashMap<Url, Result<DocumentSchema, crate::Error>>,
    catalogs: Arc<RwLock<Vec<CatalogSchema>>>,
}

impl SchemaStore {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            schemas: DashMap::new(),
            catalogs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn load_config_schema(
        &self,
        config_path: Option<std::path::PathBuf>,
        schemas: Vec<SchemaInfo>,
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

        if let Ok(mut catalogs) = self.catalogs.write() {
            for schema in schemas {
                let Ok(url) = Url::parse(&format!(
                    "file://{}",
                    config_dir.join(schema.path).to_string_lossy()
                )) else {
                    continue;
                };
                catalogs.push(CatalogSchema {
                    url,
                    include: schema.include.unwrap_or_default(),
                });
            }
        }
    }

    pub async fn load_catalog(&self, catalog_url: &url::Url) -> Result<(), crate::Error> {
        tracing::debug!("loading schema catalog: {}", catalog_url);

        if let Ok(response) = self.http_client.get(catalog_url.as_str()).send().await {
            if let Ok(catalog) = response.json::<JsonCatalog>().await {
                if let Ok(mut catalogs) = self.catalogs.write() {
                    for schema in catalog.schemas {
                        if schema
                            .file_match
                            .iter()
                            .any(|pattern| pattern.ends_with(".toml"))
                        {
                            catalogs.push(CatalogSchema {
                                url: schema.url,
                                include: schema.file_match,
                            });
                        }
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

    pub async fn get_schema_from_url<'a>(
        &'a self,
        url: &Url,
    ) -> Result<DocumentSchema, crate::Error> {
        match self.schemas.get(url) {
            Some(document_schema) => match document_schema.value() {
                Ok(document_schema) => Ok(document_schema.clone()),
                Err(err) => Err(err.clone()),
            },
            None => {
                let schema: schemars::Schema = match url.scheme() {
                    "file" => {
                        let file = std::fs::File::open(url.path()).map_err(|_| {
                            crate::Error::SchemaFileReadFailed {
                                schema_path: url.path().to_string(),
                            }
                        })?;

                        serde_json::from_reader(file)
                    }
                    "http" | "https" => {
                        let response =
                            self.http_client
                                .get(url.as_str())
                                .send()
                                .await
                                .map_err(|_| crate::Error::SchemaFetchFailed {
                                    schema_url: url.to_string(),
                                })?;

                        let bytes = response.bytes().await.map_err(|_| {
                            crate::Error::SchemaFetchFailed {
                                schema_url: url.to_string(),
                            }
                        })?;

                        serde_json::from_reader(std::io::Cursor::new(bytes))
                    }
                    _ => {
                        return Err(crate::Error::UnsupportedUrlSchema {
                            url_schema: url.scheme().to_string(),
                        })
                    }
                }
                .map_err(|_| crate::Error::SchemaFileParseFailed {
                    schema_path: url.path().to_string(),
                })?;

                let document_schema = DocumentSchema {
                    toml_version: schema
                        .get("x-tombi-toml-version")
                        .and_then(|obj| match obj {
                            serde_json::Value::String(version) => {
                                serde_json::from_str(&format!("\"{version}\"")).ok()
                            }
                            _ => None,
                        }),
                    title: schema
                        .get("title")
                        .map(|obj| obj.as_str().map(|title| title.to_string()))
                        .flatten(),
                    description: schema
                        .get("description")
                        .map(|obj| obj.as_str().map(|title| title.to_string()))
                        .flatten(),
                    schema_url: Some(url.to_owned()),
                    properties: schema
                        .get("properties")
                        .map(|obj| obj.as_object())
                        .flatten()
                        .map(|obj| {
                            obj.iter()
                                .filter_map(|(key, value)| {
                                    value.as_object().map(|_| {
                                        (Accessor::Key(key.clone()), ObjectSchema::default())
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                    ..Default::default()
                };

                self.schemas
                    .insert(url.to_owned(), Ok(document_schema.clone()));

                Ok(document_schema)
            }
        }
    }

    pub async fn get_schema_from_source(
        &self,
        source_path: &std::path::Path,
    ) -> Option<DocumentSchema> {
        let matching_urls: Vec<_> = {
            let catalogs = self.catalogs.read().ok()?;
            catalogs
                .iter()
                .filter(|catalog| {
                    catalog.include.iter().any(|pat| {
                        let pattern = if !pat.contains("*") {
                            format!("**/{}", pat)
                        } else {
                            pat.to_string()
                        };
                        glob::Pattern::new(&pattern)
                            .ok()
                            .map(|glob_pat| glob_pat.matches_path(source_path))
                            .unwrap_or(false)
                    })
                })
                .map(|catalog| catalog.url.clone())
                .collect()
        };

        for url in matching_urls {
            if let Ok(schema) = self.get_schema_from_url(&url).await {
                return Some(schema);
            }
        }
        None
    }

    pub async fn get_schema(
        &self,
        url_or_path: Either<&Url, &std::path::Path>,
    ) -> Option<DocumentSchema> {
        match url_or_path {
            Either::Left(schema_url) => {
                self.get_schema_from_url(schema_url)
                    .await
                    .ok()
                    .inspect(|_| {
                        tracing::debug!("find schema from url: {}", schema_url);
                    })
            }
            Either::Right(source_path) => {
                self.get_schema_from_source(source_path).await.inspect(|_| {
                    tracing::debug!("find schema from source: {}", source_path.display());
                })
            }
        }
    }
}
