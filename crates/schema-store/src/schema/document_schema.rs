use crate::Accessor;
use config::TomlVersion;
use indexmap::IndexMap;

use super::{referable::Referable, ValueSchema};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DocumentSchema {
    pub(crate) toml_version: Option<TomlVersion>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub schema_url: Option<url::Url>,
    pub properties: IndexMap<Accessor, Referable<ValueSchema>>,
    pub definitions: ahash::HashMap<String, Referable<ValueSchema>>,
}

impl DocumentSchema {
    pub fn toml_version(&self) -> Option<TomlVersion> {
        self.toml_version.inspect(|version| {
            tracing::debug!("use schema TOML version: {version}");
        })
    }

    pub fn resolve_ref(&self, value_schema: &mut Referable<ValueSchema>) {
        match value_schema {
            Referable::Ref(ref_str) => {
                if let Some(schema) = self.definitions.get(ref_str) {
                    let ref_str = ref_str.clone();
                    let resolved = self.resolve_ref_inner(schema);
                    if let Some(resolved) = resolved {
                        *value_schema = Referable::Resolved(resolved);
                    } else {
                        tracing::warn!("schema not found: {ref_str}");
                    }
                } else {
                    tracing::warn!("schema not found: {ref_str}");
                }
            }
            Referable::Resolved(ValueSchema::OneOf(schemas)) => {
                for schema in schemas {
                    self.resolve_ref(schema);
                }
            }
            Referable::Resolved(ValueSchema::AnyOf(schemas)) => {
                for schema in schemas {
                    self.resolve_ref(schema);
                }
            }
            Referable::Resolved(ValueSchema::AllOf(schemas)) => {
                for schema in schemas {
                    self.resolve_ref(schema);
                }
            }
            Referable::Resolved(_) => {}
        }
    }

    fn resolve_ref_inner(&self, value_schema: &Referable<ValueSchema>) -> Option<ValueSchema> {
        match value_schema {
            Referable::Ref(ref_str) => {
                if let Some(schema) = self.definitions.get(ref_str) {
                    self.resolve_ref_inner(schema)
                } else {
                    tracing::warn!("schema not found: {ref_str}");
                    None
                }
            }
            Referable::Resolved(ValueSchema::OneOf(schemas)) => Some(ValueSchema::OneOf(
                schemas
                    .iter()
                    .filter_map(|schema| self.resolve_ref_inner(schema).map(Referable::Resolved))
                    .collect(),
            )),
            Referable::Resolved(ValueSchema::AnyOf(schemas)) => Some(ValueSchema::AnyOf(
                schemas
                    .iter()
                    .filter_map(|schema| self.resolve_ref_inner(schema).map(Referable::Resolved))
                    .collect(),
            )),
            Referable::Resolved(ValueSchema::AllOf(schemas)) => Some(ValueSchema::AllOf(
                schemas
                    .iter()
                    .filter_map(|schema| self.resolve_ref_inner(schema).map(Referable::Resolved))
                    .collect(),
            )),
            Referable::Resolved(schema) => Some(schema.clone()),
        }
    }
}
