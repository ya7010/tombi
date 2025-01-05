use crate::Accessor;
use config::TomlVersion;
use indexmap::IndexMap;

use super::ValueSchema;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DocumentSchema {
    pub(crate) toml_version: Option<TomlVersion>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub schema_url: Option<url::Url>,
    pub properties: IndexMap<Accessor, ValueSchema>,
    pub definitions: ahash::HashMap<String, ValueSchema>,
}

impl DocumentSchema {
    pub fn toml_version(&self) -> Option<TomlVersion> {
        self.toml_version.inspect(|version| {
            tracing::debug!("use schema TOML version: {version}");
        })
    }

    pub fn resolve_ref(&self, value_schema: &ValueSchema) -> ValueSchema {
        match value_schema {
            ValueSchema::Ref(ref_str) => {
                if let Some(schema) = self.definitions.get(ref_str) {
                    self.resolve_ref(schema)
                } else {
                    tracing::warn!("schema not found: {ref_str}");
                    ValueSchema::Null
                }
            }
            ValueSchema::OneOf(schemas) => ValueSchema::OneOf(
                schemas
                    .iter()
                    .map(|schema| self.resolve_ref(schema))
                    .collect(),
            ),
            ValueSchema::AnyOf(schemas) => ValueSchema::AnyOf(
                schemas
                    .iter()
                    .map(|schema| self.resolve_ref(schema))
                    .collect(),
            ),
            ValueSchema::AllOf(schemas) => ValueSchema::AllOf(
                schemas
                    .iter()
                    .map(|schema| self.resolve_ref(schema))
                    .collect(),
            ),
            schema => schema.clone(),
        }
    }
}
