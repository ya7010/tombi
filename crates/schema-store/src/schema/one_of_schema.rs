use std::sync::{Arc, RwLock};

use crate::{Referable, Schemas};

use super::{SchemasTokio, ValueSchema};

#[derive(Debug, Default, Clone)]
pub struct OneOfSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub schemas: Schemas,
    pub schemas_tokio: SchemasTokio,
    pub default: Option<serde_json::Value>,
}

// FIXME: remove thoes traits.
// FIXME: remove schemas for async version
unsafe impl Send for OneOfSchema {}
unsafe impl Sync for OneOfSchema {}

impl OneOfSchema {
    pub fn new(object: &serde_json::Map<String, serde_json::Value>) -> Self {
        let title = object
            .get("title")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let description = object
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let schemas = object
            .get("oneOf")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_object())
                    .filter_map(Referable::<ValueSchema>::new)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let default = object.get("default").cloned();

        Self {
            title,
            description,
            schemas: Arc::new(RwLock::new(schemas.clone())),
            schemas_tokio: Arc::new(tokio::sync::RwLock::new(schemas.clone())),
            default,
        }
    }

    pub fn value_type(&self) -> crate::ValueType {
        crate::ValueType::OneOf(
            self.schemas
                .read()
                .unwrap()
                .iter()
                .map(|schema| schema.value_type())
                .collect(),
        )
    }
}
