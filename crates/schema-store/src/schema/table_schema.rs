use super::FindCandidates;
use super::ValueSchema;
use crate::{Accessor, Referable, SchemaProperties};
use indexmap::IndexMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default, Clone)]
pub struct TableSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub properties: SchemaProperties,
    pub required: Option<Vec<String>>,
    pub default: Option<serde_json::Value>,
}

impl TableSchema {
    pub fn new(object: &serde_json::Map<String, serde_json::Value>) -> Self {
        let mut properties = IndexMap::new();
        if let Some(serde_json::Value::Object(props)) = object.get("properties") {
            for (key, value) in props {
                let Some(object) = value.as_object() else {
                    continue;
                };
                if let Some(value_schema) = Referable::<ValueSchema>::new(&object) {
                    properties.insert(Accessor::Key(key.into()), value_schema);
                }
            }
        }

        Self {
            title: object
                .get("title")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
            description: object
                .get("description")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
            properties: Arc::new(RwLock::new(properties)),
            required: object.get("required").and_then(|v| {
                v.as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
            }),
            default: object.get("default").cloned(),
        }
    }
}

impl FindCandidates for TableSchema {
    fn find_candidates(
        &self,
        accessors: &[crate::Accessor],
        definitions: &crate::SchemaDefinitions,
    ) -> (Vec<crate::ValueSchema>, Vec<crate::Error>) {
        let mut candidates = Vec::new();
        let mut errors = Vec::new();

        let Ok(mut properties) = self.properties.write() else {
            errors.push(crate::Error::SchemaLockError);
            return (candidates, errors);
        };

        if accessors.is_empty() {
            for value in properties.values_mut() {
                if let Ok(schema) = value.resolve(definitions) {
                    let (schema_candidates, schema_errors) =
                        schema.find_candidates(accessors, definitions);
                    candidates.extend(schema_candidates);
                    errors.extend(schema_errors);
                }
            }

            return (candidates, errors);
        }

        if let Some(value) = properties.get_mut(&accessors[0]) {
            if let Ok(schema) = value.resolve(&definitions) {
                return schema.find_candidates(&accessors[1..], &definitions);
            }
        }

        (candidates, errors)
    }
}
