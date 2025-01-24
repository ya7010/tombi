use super::{referable_schema::Referable, SchemaDefinitions, ValueSchema};
use super::{FindSchemaCandidates, TableSchema};
use crate::Accessor;
use config::TomlVersion;
use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct DocumentSchema {
    pub schema_url: url::Url,
    pub schema_id: Option<url::Url>,
    pub(crate) toml_version: Option<TomlVersion>,
    table_schema: ValueSchema,
    pub definitions: SchemaDefinitions,
}

impl DocumentSchema {
    pub fn new(value: serde_json::Map<String, serde_json::Value>, schema_url: url::Url) -> Self {
        let toml_version = value.get("x-tombi-toml-version").and_then(|obj| match obj {
            serde_json::Value::String(version) => {
                serde_json::from_str(&format!("\"{version}\"")).ok()
            }
            _ => None,
        });
        let schema_id = value
            .get("$id")
            .and_then(|v| v.as_str())
            .and_then(|s| url::Url::parse(s).ok());

        let table_schema = TableSchema::new(&value);

        let definitions = DashMap::default();
        if let Some(serde_json::Value::Object(object)) = value.get("definitions") {
            for (key, value) in object.into_iter() {
                let Some(object) = value.as_object() else {
                    continue;
                };
                if let Some(value_schema) = Referable::<ValueSchema>::new(&object) {
                    definitions.insert(format!("#/definitions/{key}"), value_schema);
                }
            }
        }
        if let Some(serde_json::Value::Object(object)) = value.get("$defs") {
            for (key, value) in object.into_iter() {
                let Some(object) = value.as_object() else {
                    continue;
                };
                if let Some(value_schema) = Referable::<ValueSchema>::new(&object) {
                    definitions.insert(format!("#/$defs/{key}"), value_schema);
                }
            }
        }

        Self {
            schema_url,
            schema_id,
            toml_version,
            table_schema: ValueSchema::Table(table_schema),
            definitions,
        }
    }

    pub fn toml_version(&self) -> Option<TomlVersion> {
        self.toml_version.inspect(|version| {
            tracing::debug!("use schema TOML version: {version}");
        })
    }

    pub fn value_type(&self) -> crate::ValueType {
        crate::ValueType::Table
    }

    pub fn table_schema(&self) -> &TableSchema {
        match &self.table_schema {
            ValueSchema::Table(table_schema) => table_schema,
            _ => unreachable!(),
        }
    }

    pub fn value_schema(&self) -> &ValueSchema {
        &self.table_schema
    }
}

impl FindSchemaCandidates for DocumentSchema {
    fn find_schema_candidates(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
    ) -> (Vec<ValueSchema>, Vec<crate::Error>) {
        self.table_schema
            .find_schema_candidates(accessors, definitions)
    }
}
