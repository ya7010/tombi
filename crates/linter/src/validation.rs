mod boolean;
mod table;
mod value;

use config::TomlVersion;
use schema_store::SchemaDefinitions;
use schema_store::ValueSchema;
use std::ops::Deref;

trait Validate {
    fn validate(
        &self,
        toml_version: TomlVersion,
        value_schema: &ValueSchema,
        definitions: &SchemaDefinitions,
    ) -> Result<(), Vec<crate::Error>>;
}

pub fn validate(
    root: document_tree::Root,
    toml_version: TomlVersion,
    document_schema: schema_store::DocumentSchema,
) -> Result<(), Vec<crate::Error>> {
    let table = root.deref();
    let (value_schema, definitions) = document_schema.into();

    table.validate(toml_version, &value_schema, &definitions)
}
