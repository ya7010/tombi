use std::{borrow::Cow, fmt::Debug};

use diagnostic::SetDiagnostics;
use document_tree::ValueImpl;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{CurrentSchema, ValueSchema};

use super::Validate;
use crate::validation::{all_of::validate_all_of, one_of::validate_one_of};

pub fn validate_any_of<'a: 'b, 'b, T>(
    value: &'a T,
    accessors: &'a [schema_store::SchemaAccessor],
    any_of_schema: &'a schema_store::AnyOfSchema,
    schema_url: &'a schema_store::SchemaUrl,
    definitions: &'a schema_store::SchemaDefinitions,
    schema_context: &'a schema_store::SchemaContext<'a>,
) -> BoxFuture<'b, Result<(), Vec<diagnostic::Diagnostic>>>
where
    T: Validate + ValueImpl + Sync + Send + Debug,
{
    tracing::trace!("value = {:?}", value);
    tracing::trace!("any_of_schema = {:?}", any_of_schema);

    async move {
        let mut diagnostics = vec![];
        let mut is_type_match = false;
        let mut type_mismatch_errors = vec![];

        let mut schemas = any_of_schema.schemas.write().await;
        for referable_schema in schemas.iter_mut() {
            let Ok(Some(CurrentSchema {
                value_schema,
                schema_url,
                definitions,
            })) = referable_schema
                .resolve(
                    Cow::Borrowed(schema_url),
                    Cow::Borrowed(definitions),
                    schema_context.store,
                )
                .await
            else {
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
                    match value
                        .validate(
                            accessors,
                            Some(value_schema),
                            Some(&schema_url),
                            Some(&definitions),
                            schema_context,
                        )
                        .await
                    {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_diagnostics) => diagnostics.append(&mut schema_diagnostics),
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
                        accessors,
                        one_of_schema,
                        &schema_url,
                        &definitions,
                        schema_context,
                    )
                    .await
                    {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_diagnostics) => diagnostics.append(&mut schema_diagnostics),
                    }
                }
                (_, ValueSchema::AnyOf(any_of_schema)) => {
                    match validate_any_of(
                        value,
                        accessors,
                        any_of_schema,
                        &schema_url,
                        &definitions,
                        schema_context,
                    )
                    .await
                    {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_diagnostics) => diagnostics.append(&mut schema_diagnostics),
                    }
                }
                (_, ValueSchema::AllOf(all_of_schema)) => {
                    match validate_all_of(
                        value,
                        accessors,
                        all_of_schema,
                        &schema_url,
                        &definitions,
                        schema_context,
                    )
                    .await
                    {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(mut schema_diagnostics) => diagnostics.append(&mut schema_diagnostics),
                    }
                }
            }
        }

        if !is_type_match {
            type_mismatch_errors.set_diagnostics(&mut diagnostics);
        }

        Err(diagnostics)
    }
    .boxed()
}
