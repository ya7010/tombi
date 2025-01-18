use config::TomlVersion;
use schema_store::{SchemaDefinitions, ValueSchema};

use super::Validate;

impl Validate for document_tree::Value {
    fn validate(
        &self,
        toml_version: TomlVersion,
        value_schema: &ValueSchema,
        definitions: &SchemaDefinitions,
    ) -> Result<(), Vec<crate::Error>> {
        match self {
            Self::Boolean(boolean) => boolean.validate(toml_version, value_schema, definitions),
            Self::Integer(integer) => integer.validate(toml_version, value_schema, definitions),
            Self::Float(float) => float.validate(toml_version, value_schema, definitions),
            Self::String(string) => string.validate(toml_version, value_schema, definitions),
            Self::Table(table) => table.validate(toml_version, value_schema, definitions),
            _ => Ok(()),
        }
    }
}
