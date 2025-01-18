mod boolean;
mod float;
mod integer;
mod string;
mod table;
mod value;

use config::TomlVersion;
use schema_store::OneOfSchema;
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

fn validate_one_of<T>(
    value: &T,
    toml_version: TomlVersion,
    one_of_schema: &OneOfSchema,
    definitions: &SchemaDefinitions,
) -> Result<(), Vec<crate::Error>>
where
    T: Validate,
{
    let mut errors = vec![];

    let mut valid_count = 0;

    if let Ok(mut schemas) = one_of_schema.schemas.write() {
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            match value.validate(toml_version, value_schema, definitions) {
                Ok(()) => {
                    valid_count += 1;
                    break;
                }
                Err(mut schema_errors) => errors.append(&mut schema_errors),
            }
        }
    }

    if valid_count == 1 {
        return Ok(());
    }

    Err(errors
        .into_iter()
        .filter(|error| !matches!(error.kind, crate::ErrorKind::TypeMismatch { .. }))
        .collect())
}

fn validate_any_of<T>(
    value: &T,
    toml_version: TomlVersion,
    any_of_schema: &schema_store::AnyOfSchema,
    definitions: &schema_store::SchemaDefinitions,
) -> Result<(), Vec<crate::Error>>
where
    T: Validate,
{
    let mut errors = vec![];

    if let Ok(mut schemas) = any_of_schema.schemas.write() {
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            match value.validate(toml_version, value_schema, definitions) {
                Ok(()) => {
                    return Ok(());
                }
                Err(mut schema_errors) => errors.append(&mut schema_errors),
            }
        }
    }

    Err(errors
        .into_iter()
        .filter(|error| !matches!(error.kind, crate::ErrorKind::TypeMismatch { .. }))
        .collect())
}

fn validate_all_of<T>(
    value: &T,
    toml_version: TomlVersion,
    all_of_schema: &schema_store::AllOfSchema,
    definitions: &schema_store::SchemaDefinitions,
) -> Result<(), Vec<crate::Error>>
where
    T: Validate,
{
    let mut errors = vec![];

    if let Ok(mut schemas) = all_of_schema.schemas.write() {
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            match value.validate(toml_version, value_schema, definitions) {
                Ok(()) => {}
                Err(mut schema_errors) => errors.append(&mut schema_errors),
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
