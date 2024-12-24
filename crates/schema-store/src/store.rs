use dashmap::DashMap;
use url::Url;

use crate::DocumentSchema;

#[derive(Debug, Default)]
pub struct SchemaStore {
    // http_client: reqwest::Client,
    schemas: DashMap<Url, DocumentSchema>,
}

impl SchemaStore {
    pub fn new() -> Self {
        Self {
            // http_client: reqwest::Client::new(),
            schemas: DashMap::new(),
        }
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
