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
            document_tree::Value::Table(table) => {
                table.validate(errors, toml_version, value_schema, definitions)
            }
            _ => {}
        }
    }
}
