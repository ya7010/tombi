pub mod table;
pub mod value;

use schema_store::SchemaDefinitions;
use schema_store::ValueSchema;
use std::ops::Deref;

trait Validate {
    fn validate(
        &self,
        errors: &mut Vec<crate::Error>,
        value_schema: &ValueSchema,
        definitions: &SchemaDefinitions,
    );
}

pub fn validate(
    root: document_tree::Root,
    document_schema: schema_store::DocumentSchema,
) -> Result<(), Vec<crate::Error>> {
    let mut errors = Vec::new();

    let table = root.deref();
    let (value_schema, definitions) = document_schema.into();

    table.validate(&mut errors, &value_schema, &definitions);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
