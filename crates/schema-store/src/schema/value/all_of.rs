use crate::Referable;

use super::ValueSchema;

#[derive(Debug, Default, Clone)]
pub struct AllOfSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub schemas: Vec<Referable<ValueSchema>>,
    pub default: Option<serde_json::Value>,
}

impl AllOfSchema {
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
            .get("allOf")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_object())
                    .filter_map(|object| Referable::<ValueSchema>::new(object))
                    .collect()
            })
            .unwrap_or_default();
        let default = object.get("default").cloned();

        Self {
            title,
            description,
            schemas,
            default,
        }
    }
}
