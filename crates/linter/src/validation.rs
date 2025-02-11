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
use futures::future::BoxFuture;
use futures::FutureExt;
use schema_store::OneOfSchema;
use schema_store::SchemaDefinitions;
use schema_store::ValueSchema;
use std::ops::Deref;

trait Validate {
    fn validate<'a: 'b, 'b>(
        &'a self,
        toml_version: TomlVersion,
        value_schema: &'a ValueSchema,
        definitions: &'a SchemaDefinitions,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Result<(), Vec<crate::Error>>>;
}

pub fn validate<'a: 'b, 'b>(
    tree: document_tree::DocumentTree,
    toml_version: TomlVersion,
    document_schema: &'a schema_store::DocumentSchema,
    schema_store: &'a schema_store::SchemaStore,
) -> BoxFuture<'b, Result<(), Vec<crate::Error>>> {
    async move {
        let table = tree.deref();

        if let Some(value_schema) = &document_schema.value_schema {
            table
                .validate(
                    toml_version,
                    value_schema,
                    &document_schema.definitions,
                    &schema_store,
                )
                .await?;
        }

        Ok(())
    }
    .boxed()
}

fn validate_one_of<'a: 'b, 'b, T>(
    value: &'a T,
    toml_version: TomlVersion,
    one_of_schema: &'a OneOfSchema,
    definitions: &'a SchemaDefinitions,
    schema_store: &'a schema_store::SchemaStore,
) -> BoxFuture<'b, Result<(), Vec<crate::Error>>>
where
    T: Validate + ValueImpl + Sync + Send,
{
    async move {
        let mut errors = vec![];
        let mut is_type_match = false;
        let mut type_mismatch_errors = vec![];
        let mut valid_count = 0;

        let mut schemas = one_of_schema.schemas.write().await;
        for referable_schema in schemas.iter_mut() {
            let Ok((value_schema, new_schema)) = referable_schema
                .resolve(definitions, &schema_store)
                .await
            else {
                continue;
            };

            let definitions = if let Some((_, definitions)) = &new_schema {
                definitions
            } else {
                definitions
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
                    match value
                        .validate(toml_version, value_schema, definitions, schema_store)
                        .await
                    {
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
                            expected: value_schema.value_type().await,
                            actual: value.value_type(),
                        },
                        range: value.range(),
                    });
                }
                (_, ValueSchema::OneOf(one_of_schema)) => {
                    match validate_one_of(
                        value,
                        toml_version,
                        one_of_schema,
                        definitions,
                        &schema_store,
                    )
                    .await
                    {
                        Ok(()) => {
                            valid_count += 1;
                            break;
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::AnyOf(any_of_schema)) => {
                    match validate_any_of(
                        value,
                        toml_version,
                        any_of_schema,
                        definitions,
                        &schema_store,
                    )
                    .await
                    {
                        Ok(()) => {
                            valid_count += 1;
                            break;
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::AllOf(all_of_schema)) => {
                    match validate_all_of(
                        value,
                        toml_version,
                        all_of_schema,
                        definitions,
                        &schema_store,
                    )
                    .await
                    {
                        Ok(()) => {
                            valid_count += 1;
                            break;
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
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
    .boxed()
}

fn validate_any_of<'a: 'b, 'b, T>(
    value: &'a T,
    toml_version: TomlVersion,
    any_of_schema: &'a schema_store::AnyOfSchema,
    definitions: &'a schema_store::SchemaDefinitions,
    schema_store: &'a schema_store::SchemaStore,
) -> BoxFuture<'b, Result<(), Vec<crate::Error>>>
where
    T: Validate + ValueImpl + Sync + Send,
{
    async move {
        let mut errors = vec![];
        let mut is_type_match = false;
        let mut type_mismatch_errors = vec![];

        let mut schemas = any_of_schema.schemas.write().await;
        for referable_schema in schemas.iter_mut() {
            let Ok((value_schema, new_schema)) = referable_schema
                .resolve(definitions, &schema_store)
                .await
            else {
                continue;
            };

            let definitions = if let Some((_, definitions)) = &new_schema {
                definitions
            } else {
                definitions
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
                    match value
                        .validate(toml_version, value_schema, definitions, &schema_store)
                        .await
                    {
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
                            expected: value_schema.value_type().await,
                            actual: value.value_type(),
                        },
                        range: value.range(),
                    });
                }
                (_, ValueSchema::OneOf(one_of_schema)) => {
                    match validate_one_of(
                        value,
                        toml_version,
                        one_of_schema,
                        definitions,
                        &schema_store,
                    )
                    .await
                    {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::AnyOf(any_of_schema)) => {
                    match validate_any_of(
                        value,
                        toml_version,
                        any_of_schema,
                        definitions,
                        &schema_store,
                    )
                    .await
                    {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::AllOf(all_of_schema)) => {
                    match validate_all_of(
                        value,
                        toml_version,
                        all_of_schema,
                        definitions,
                        &schema_store,
                    )
                    .await
                    {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_errors) => errors.append(&mut schema_errors),
                    }
                }
            }
        }

        if !is_type_match {
            errors.append(&mut type_mismatch_errors);
        }

        Err(errors)
    }
    .boxed()
}

fn validate_all_of<'a: 'b, 'b, T>(
    value: &'a T,
    toml_version: TomlVersion,
    all_of_schema: &'a schema_store::AllOfSchema,
    definitions: &'a schema_store::SchemaDefinitions,
    schema_store: &'a schema_store::SchemaStore,
) -> BoxFuture<'b, Result<(), Vec<crate::Error>>>
where
    T: Validate + Sync + Send,
{
    async move {
        let mut errors = vec![];

        let mut schemas = all_of_schema.schemas.write().await;
        for referable_schema in schemas.iter_mut() {
            let Ok((value_schema, new_schema)) = referable_schema
                .resolve(definitions, schema_store)
                .await
            else {
                continue;
            };

            let definitions = if let Some((_, definitions)) = &new_schema {
                definitions
            } else {
                definitions
            };

            match value
                .validate(toml_version, value_schema, definitions, &schema_store)
                .await
            {
                Ok(()) => {}
                Err(mut schema_errors) => errors.append(&mut schema_errors),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    .boxed()
}
