use crate::Accessor;
use config::TomlVersion;
use indexmap::IndexMap;
use std::sync::{Arc, RwLock};

use super::{referable::Referable, ValueSchema};

#[derive(Debug, Clone)]
pub struct DocumentSchema {
    pub(crate) toml_version: Option<TomlVersion>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub schema_url: Option<url::Url>,
    pub properties: Arc<RwLock<IndexMap<Accessor, Referable<ValueSchema>>>>,
    pub definitions: Arc<RwLock<ahash::HashMap<String, Referable<ValueSchema>>>>,
}

impl DocumentSchema {
    pub fn new(content: serde_json::Value) -> Self {
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
            toml_version,
            title,
            description,
            schema_url,
            properties: Arc::new(RwLock::new(properties)),
            definitions: Arc::new(RwLock::new(definitions)),
        }
    }

    pub fn toml_version(&self) -> Option<TomlVersion> {
        self.toml_version.inspect(|version| {
            tracing::debug!("use schema TOML version: {version}");
        })
    }

    pub fn find_schema_candidates(&self, accessors: &[Accessor]) -> Vec<ValueSchema> {
        let candidates = Vec::new();
        let mut _properties = self.properties.write().unwrap();

        if accessors.is_empty() {
            return candidates;
        }

        candidates
    }
}

impl PartialEq for DocumentSchema {
    fn eq(&self, other: &Self) -> bool {
        let props_eq = if let (Ok(self_props), Ok(other_props)) =
            (self.properties.read(), other.properties.read())
        {
            *self_props == *other_props
        } else {
            false
        };

        let defs_eq = if let (Ok(self_defs), Ok(other_defs)) =
            (self.definitions.read(), other.definitions.read())
        {
            *self_defs == *other_defs
        } else {
            false
        };

        self.toml_version == other.toml_version
            && self.title == other.title
            && self.description == other.description
            && self.schema_url == other.schema_url
            && props_eq
            && defs_eq
    }
}
