use dashmap::DashMap;
use url::Url;

use crate::ValueSchema;

pub struct Store {
    // http_client: reqwest::Client,
    schemas: DashMap<Url, ValueSchema>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            // http_client: reqwest::Client::new(),
            schemas: DashMap::new(),
        }
    }

    pub fn add_schema(&mut self, url: Url, schema: ValueSchema) {
        self.schemas.insert(url, schema);
    }

    pub fn get_schema(&self, url: &Url) -> Option<ValueSchema> {
        match self.schemas.get(url) {
            Some(schema) => Some(schema.clone()),
            None => None,
        }
    }
}
