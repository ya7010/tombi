use schema_store::{SchemaDefinitions, ValueSchema};

use super::Validate;

impl Validate for document_tree::Value {
    fn validate(
        &self,
        errors: &mut Vec<crate::Error>,
        value_schema: &ValueSchema,
        definitions: &SchemaDefinitions,
    ) {
        match self {
            document_tree::Value::Table(table) => table.validate(errors, value_schema, definitions),
            _ => {}
        }
    }
}
