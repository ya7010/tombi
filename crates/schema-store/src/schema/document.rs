use super::{referable::Referable, ValueSchema};
use crate::Accessor;
use config::TomlVersion;
use indexmap::IndexMap;
use std::sync::{Arc, RwLock};

pub type SchemaProperties = Arc<RwLock<IndexMap<Accessor, Referable<ValueSchema>>>>;
pub type SchemaDefinitions = Arc<RwLock<ahash::HashMap<String, Referable<ValueSchema>>>>;

#[derive(Debug, Clone)]
pub struct DocumentSchema {
    pub document_url: url::Url,
    pub schema_id: Option<url::Url>,
    pub(crate) toml_version: Option<TomlVersion>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub properties: SchemaProperties,
    pub definitions: SchemaDefinitions,
}

impl DocumentSchema {
    pub fn new(content: serde_json::Value, document_url: url::Url) -> Self {
        let toml_version = content
            .get("x-tombi-toml-version")
            .and_then(|obj| match obj {
                serde_json::Value::String(version) => {
                    serde_json::from_str(&format!("\"{version}\"")).ok()
                }
                _ => None,
            });
        let title = content
            .get("title")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let description = content
            .get("description")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        let schema_url = content
            .get("$id")
            .and_then(|v| v.as_str())
            .and_then(|s| url::Url::parse(s).ok());

        let mut properties = IndexMap::default();
        if content.get("properties").is_some() {
            if let Some(serde_json::Value::Object(object)) = content.get("properties") {
                for (key, value) in object.into_iter() {
                    let Some(object) = value.as_object() else {
                        continue;
                    };
                    if let Some(value_schema) = Referable::<ValueSchema>::new(&object) {
                        properties.insert(Accessor::Key(key.into()), value_schema);
                    }
                }
            }
        }

        let mut definitions = ahash::HashMap::default();
        if content.get("definitions").is_some() {
            if let Some(serde_json::Value::Object(object)) = content.get("definitions") {
                for (key, value) in object.into_iter() {
                    let Some(object) = value.as_object() else {
                        continue;
                    };
                    if let Some(value_schema) = Referable::<ValueSchema>::new(&object) {
                        definitions.insert(format!("#/definitions/{key}"), value_schema);
                    }
                }
            }
        }
        if content.get("$defs").is_some() {
            if let Some(serde_json::Value::Object(object)) = content.get("$defs") {
                for (key, value) in object.into_iter() {
                    let Some(object) = value.as_object() else {
                        continue;
                    };
                    if let Some(value_schema) = Referable::<ValueSchema>::new(&object) {
                        definitions.insert(format!("#/$defs/{key}"), value_schema);
                    }
                }
            }
        }

        Self {
            document_url,
            schema_id: schema_url,
            toml_version,
            title,
            description,
            properties: Arc::new(RwLock::new(properties)),
            definitions: Arc::new(RwLock::new(definitions)),
        }
    }

    pub fn toml_version(&self) -> Option<TomlVersion> {
        self.toml_version.inspect(|version| {
            tracing::debug!("use schema TOML version: {version}");
        })
    }
}
