mod array;
mod boolean;
mod float;
mod integer;
mod local_date;
mod local_date_time;
mod local_time;
mod offset_date_time;
mod string;
mod table;
mod value;

use config::TomlVersion;
use document_tree::ValueImpl;
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
    tree: document_tree::DocumentTree,
    toml_version: TomlVersion,
    document_schema: &schema_store::DocumentSchema,
) -> Result<(), Vec<crate::Error>> {
    let table = tree.deref();

    table.validate(
        toml_version,
        document_schema.value_schema(),
        &document_schema.definitions,
    )
}

fn validate_one_of<T>(
    value: &T,
    toml_version: TomlVersion,
    one_of_schema: &OneOfSchema,
    definitions: &SchemaDefinitions,
) -> Result<(), Vec<crate::Error>>
where
    T: Validate + ValueImpl,
{
    let mut errors = vec![];
    let mut is_type_match = false;
    let mut type_mismatch_errors = vec![];
    let mut valid_count = 0;

    if let Ok(mut schemas) = one_of_schema.schemas.write() {
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };

            match (value.value_type(), value_schema) {
                (document_tree::ValueType::Boolean, ValueSchema::Boolean(_))
                | (document_tree::ValueType::Integer, ValueSchema::Integer(_))
                | (document_tree::ValueType::Float, ValueSchema::Float(_))
                | (document_tree::ValueType::String, ValueSchema::String(_))
                | (document_tree::ValueType::OffsetDateTime, ValueSchema::OffsetDateTime(_))
                | (document_tree::ValueType::LocalDateTime, ValueSchema::LocalDateTime(_))
                | (document_tree::ValueType::LocalDate, ValueSchema::LocalDate(_))
                | (document_tree::ValueType::LocalTime, ValueSchema::LocalTime(_))
                | (document_tree::ValueType::Table, ValueSchema::Table(_))
                | (document_tree::ValueType::Array, ValueSchema::Array(_)) => {
                    is_type_match = true;
                    match value.validate(toml_version, value_schema, definitions) {
                        Ok(()) => {
                            valid_count += 1;
                            break;
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::Boolean(_))
                | (_, ValueSchema::Integer(_))
                | (_, ValueSchema::Float(_))
                | (_, ValueSchema::String(_))
                | (_, ValueSchema::OffsetDateTime(_))
                | (_, ValueSchema::LocalDateTime(_))
                | (_, ValueSchema::LocalDate(_))
                | (_, ValueSchema::LocalTime(_))
                | (_, ValueSchema::Table(_))
                | (_, ValueSchema::Array(_))
                | (_, ValueSchema::Null) => {
                    type_mismatch_errors.push(crate::Error {
                        kind: crate::ErrorKind::TypeMismatch {
                            expected: value_schema.value_type(),
                            actual: value.value_type(),
                        },
                        range: value.range(),
                    });
                }
                (_, ValueSchema::OneOf(one_of_schema)) => {
                    match validate_one_of(value, toml_version, one_of_schema, definitions) {
                        Ok(()) => {
                            valid_count += 1;
                            break;
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::AnyOf(any_of_schema)) => {
                    match validate_any_of(value, toml_version, any_of_schema, definitions) {
                        Ok(()) => {
                            valid_count += 1;
                            break;
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::AllOf(all_of_schema)) => {
                    match validate_all_of(value, toml_version, all_of_schema, definitions) {
                        Ok(()) => {
                            valid_count += 1;
                            break;
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
            }
        }
    }

    if valid_count == 1 {
        return Ok(());
    }

    if !is_type_match {
        errors.append(&mut type_mismatch_errors);
    }

    Err(errors)
}

fn validate_any_of<T>(
    value: &T,
    toml_version: TomlVersion,
    any_of_schema: &schema_store::AnyOfSchema,
    definitions: &schema_store::SchemaDefinitions,
) -> Result<(), Vec<crate::Error>>
where
    T: Validate + ValueImpl,
{
    let mut errors = vec![];
    let mut is_type_match = false;
    let mut type_mismatch_errors = vec![];

    if let Ok(mut schemas) = any_of_schema.schemas.write() {
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            match (value.value_type(), value_schema) {
                (document_tree::ValueType::Boolean, ValueSchema::Boolean(_))
                | (document_tree::ValueType::Integer, ValueSchema::Integer(_))
                | (document_tree::ValueType::Float, ValueSchema::Float(_))
                | (document_tree::ValueType::String, ValueSchema::String(_))
                | (document_tree::ValueType::OffsetDateTime, ValueSchema::OffsetDateTime(_))
                | (document_tree::ValueType::LocalDateTime, ValueSchema::LocalDateTime(_))
                | (document_tree::ValueType::LocalDate, ValueSchema::LocalDate(_))
                | (document_tree::ValueType::LocalTime, ValueSchema::LocalTime(_))
                | (document_tree::ValueType::Table, ValueSchema::Table(_))
                | (document_tree::ValueType::Array, ValueSchema::Array(_)) => {
                    is_type_match = true;
                    match value.validate(toml_version, value_schema, definitions) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::Boolean(_))
                | (_, ValueSchema::Integer(_))
                | (_, ValueSchema::Float(_))
                | (_, ValueSchema::String(_))
                | (_, ValueSchema::OffsetDateTime(_))
                | (_, ValueSchema::LocalDateTime(_))
                | (_, ValueSchema::LocalDate(_))
                | (_, ValueSchema::LocalTime(_))
                | (_, ValueSchema::Table(_))
                | (_, ValueSchema::Array(_))
                | (_, ValueSchema::Null) => {
                    type_mismatch_errors.push(crate::Error {
                        kind: crate::ErrorKind::TypeMismatch {
                            expected: value_schema.value_type(),
                            actual: value.value_type(),
                        },
                        range: value.range(),
                    });
                }
                (_, ValueSchema::OneOf(one_of_schema)) => {
                    match validate_one_of(value, toml_version, one_of_schema, definitions) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::AnyOf(any_of_schema)) => {
                    match validate_any_of(value, toml_version, any_of_schema, definitions) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::AllOf(all_of_schema)) => {
                    match validate_all_of(value, toml_version, all_of_schema, definitions) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
            }
        }
    }

    if !is_type_match {
        errors.append(&mut type_mismatch_errors);
    }

    Err(errors)
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
