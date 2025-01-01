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
        config_dirpath: Option<std::path::PathBuf>,
        schemas: &[SchemaInfo],
    ) {
        let config_dirpath = match config_dirpath {
            Some(path) => path,
            None => std::env::current_dir().unwrap(),
        };
        if let Ok(mut catalogs) = self.catalogs.write() {
            for schema in schemas.iter() {
                let Ok(url) = Url::parse(&format!(
                    "file://{}",
                    config_dirpath.join(&schema.path).to_string_lossy()
                )) else {
                    continue;
                };
                tracing::debug!("load config schema from: {}", url);

                catalogs.push(CatalogSchema {
                    url,
                    include: match schema.include.as_ref() {
                        Some(include) => include.clone(),
                        None => Vec::with_capacity(0),
                    },
                });
            }
        }
    }

    pub async fn update_schema(&self, schema_url: &Url) -> Result<bool, crate::Error> {
        if self.schemas.contains_key(schema_url) {
            let document_schema = self.try_get_schema_from_schema_url(schema_url).await?;

            self.schemas.insert(schema_url.clone(), Ok(document_schema));
            tracing::debug!("update schema: {}", schema_url);

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn load_catalog_from_url(&self, catalog_url: &url::Url) -> Result<(), crate::Error> {
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

    async fn try_get_schema_from_schema_url(
        &self,
        schema_url: &Url,
    ) -> Result<DocumentSchema, crate::Error> {
        let schema: schemars::Schema = match schema_url.scheme() {
            "file" => {
                let file = std::fs::File::open(schema_url.path()).map_err(|_| {
                    crate::Error::SchemaFileReadFailed {
                        schema_path: schema_url.path().to_string(),
                    }
                })?;

                serde_json::from_reader(file)
            }
            "http" | "https" => {
                let response = self
                    .http_client
                    .get(schema_url.as_str())
                    .send()
                    .await
                    .map_err(|_| crate::Error::SchemaFetchFailed {
                        schema_url: schema_url.to_string(),
                    })?;

                let bytes =
                    response
                        .bytes()
                        .await
                        .map_err(|_| crate::Error::SchemaFetchFailed {
                            schema_url: schema_url.to_string(),
                        })?;

                serde_json::from_reader(std::io::Cursor::new(bytes))
            }
            _ => {
                return Err(crate::Error::UnsupportedUrlSchema {
                    schema_url: schema_url.to_owned(),
                })
            }
        }
        .map_err(|_| crate::Error::SchemaFileParseFailed {
            schema_url: schema_url.to_owned(),
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
            schema_url: Some(schema_url.to_owned()),
            properties: schema
                .get("properties")
                .map(|obj| obj.as_object())
                .flatten()
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(key, value)| {
                            value
                                .as_object()
                                .map(|_| (Accessor::Key(key.clone()), ObjectSchema::default()))
                        })
                        .collect()
                })
                .unwrap_or_default(),
            ..Default::default()
        };

        Ok(document_schema)
    }

    async fn try_load_schema(&self, schema_url: &Url) -> Result<DocumentSchema, crate::Error> {
        match self.schemas.get(schema_url) {
            Some(document_schema) => match document_schema.value() {
                Ok(document_schema) => Ok(document_schema.clone()),
                Err(err) => Err(err.clone()),
            },
            None => {
                let document_schema = self.try_get_schema_from_schema_url(schema_url).await?;

                self.schemas
                    .insert(schema_url.to_owned(), Ok(document_schema.clone()));

                Ok(document_schema)
            }
        }
    }

    pub async fn try_get_schema_from_url(
        &self,
        source_url: &url::Url,
    ) -> Result<Option<DocumentSchema>, crate::Error> {
        match source_url.scheme() {
            "file" => {
                let source_path = source_url.path();
                let source_path = std::path::Path::new(source_path);
                self.try_get_schema_from_path(source_path).await
            }
            _ => Err(crate::Error::UnsupportedUrlSchema {
                schema_url: source_url.to_owned(),
            }),
        }
    }

    pub async fn try_get_schema_from_path(
        &self,
        source_path: &std::path::Path,
    ) -> Result<Option<DocumentSchema>, crate::Error> {
        let matching_schema_urls: Vec<_> = {
            match self.catalogs.read() {
                Ok(catalogs) => catalogs
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
                    .collect(),
                Err(_) => Vec::with_capacity(0),
            }
        };

        for schema_url in matching_schema_urls {
            return Ok(Some(self.try_load_schema(&schema_url).await?));
        }
        Ok(None)
    }

    pub async fn try_get_schema(
        &self,
        source_url_or_path: Either<&Url, &std::path::Path>,
    ) -> Result<Option<DocumentSchema>, crate::Error> {
        match source_url_or_path {
            Either::Left(source_url) => {
                self.try_get_schema_from_url(source_url).await.inspect(|_| {
                    tracing::debug!("find schema from url: {}", source_url);
                })
            }
            Either::Right(source_path) => {
                self.try_get_schema_from_path(source_path)
                    .await
                    .inspect(|_| {
                        tracing::debug!("find schema from source: {}", source_path.display());
                    })
            }
        }
    }

    pub fn catalog_urls(&self) -> Vec<Url> {
        match self.catalogs.read() {
            Ok(catalogs) => catalogs
                .iter()
                .map(|catalog| catalog.url.clone())
                .collect::<Vec<_>>(),
            Err(_) => Vec::with_capacity(0),
        }
    }
}
