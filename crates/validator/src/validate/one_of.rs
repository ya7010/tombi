use std::fmt::Debug;

use diagnostic::SetDiagnostics;
use document_tree::ValueImpl;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{CurrentSchema, OneOfSchema, ValueSchema};

use super::Validate;
use crate::validate::{all_of::validate_all_of, any_of::validate_any_of};

pub fn validate_one_of<'a: 'b, 'b, T>(
    value: &'a T,
    accessors: &'a [schema_store::SchemaAccessor],
    one_of_schema: &'a OneOfSchema,
    current_schema: &'a CurrentSchema<'a>,
    schema_context: &'a schema_store::SchemaContext<'a>,
) -> BoxFuture<'b, Result<(), Vec<diagnostic::Diagnostic>>>
where
    T: Validate + ValueImpl + Sync + Send + Debug,
{
    tracing::trace!("value = {:?}", value);
    tracing::trace!("one_of_schema = {:?}", one_of_schema);

    async move {
        let mut diagnostics = vec![];
        let mut is_type_match = false;
        let mut type_mismatch_errors = vec![];
        let mut valid_count = 0;

        let mut schemas = one_of_schema.schemas.write().await;
        for referable_schema in schemas.iter_mut() {
            let Ok(Some(current_schema)) = referable_schema
                .resolve(
                    current_schema.schema_url.clone(),
                    current_schema.definitions.clone(),
                    schema_context.store,
                )
                .await
            else {
                continue;
            };

            match (value.value_type(), current_schema.value_schema.as_ref()) {
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
                        .validate(accessors, Some(&current_schema), schema_context)
                        .await
                    {
                        Ok(()) => {
                            valid_count += 1;
                            break;
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
                            expected: current_schema.value_schema.value_type().await,
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
                        &current_schema,
                        schema_context,
                    )
                    .await
                    {
                        Ok(()) => {
                            valid_count += 1;
                            break;
                        }
                        Err(mut schema_errors) => diagnostics.append(&mut schema_errors),
                    }
                }
                (_, ValueSchema::AnyOf(any_of_schema)) => {
                    match validate_any_of(
                        value,
                        accessors,
                        any_of_schema,
                        &current_schema,
                        schema_context,
                    )
                    .await
                    {
                        Ok(()) => {
                            valid_count += 1;
                            break;
                        }
                        Err(mut schema_diagnostics) => diagnostics.append(&mut schema_diagnostics),
                    }
                }
                (_, ValueSchema::AllOf(all_of_schema)) => {
                    match validate_all_of(
                        value,
                        accessors,
                        all_of_schema,
                        &current_schema,
                        schema_context,
                    )
                    .await
                    {
                        Ok(()) => {
                            valid_count += 1;
                            break;
                        }
                        Err(mut schema_diagnostics) => diagnostics.append(&mut schema_diagnostics),
                    }
                }
            }
        }

        if valid_count == 1 {
            return Ok(());
        }

        if !is_type_match {
            type_mismatch_errors.set_diagnostics(&mut diagnostics);
        }

        Err(diagnostics)
    }
    .boxed()
}
