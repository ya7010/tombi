use config::TomlVersion;
use schema_store::{SchemaDefinitions, ValueSchema};

use super::Validate;

impl Validate for document_tree::Value {
    fn validate(
        &self,
        errors: &mut Vec<crate::Error>,
        toml_version: TomlVersion,
        value_schema: &ValueSchema,
        definitions: &SchemaDefinitions,
    ) {
        match self {
            Self::Boolean(boolean) => {
                boolean.validate(errors, toml_version, value_schema, definitions)
            }
            Self::Table(table) => table.validate(errors, toml_version, value_schema, definitions),
            _ => {}
        }
    }
}
