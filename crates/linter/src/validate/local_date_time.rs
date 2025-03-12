use diagnostic::SetDiagnostics;
use document_tree::{LocalDateTime, ValueImpl};
use futures::{future::BoxFuture, FutureExt};
use schema_store::ValueType;

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for LocalDateTime {
    fn validate<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::SchemaAccessor],
        value_schema: Option<&'a schema_store::ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext,
    ) -> BoxFuture<'b, Result<(), Vec<diagnostic::Diagnostic>>> {
        async move {
            let mut diagnostics = vec![];

            match (value_schema, schema_url, definitions) {
                (Some(value_schema), Some(schema_url), Some(definitions)) => {
                    match value_schema.value_type().await {
                        ValueType::LocalDateTime
                        | ValueType::OneOf(_)
                        | ValueType::AnyOf(_)
                        | ValueType::AllOf(_) => {}
                        ValueType::Null => return Ok(()),
                        value_schema => {
                            crate::Error {
                                kind: crate::ErrorKind::TypeMismatch {
                                    expected: value_schema,
                                    actual: self.value_type(),
                                },
                                range: self.range(),
                            }
                            .set_diagnostics(&mut diagnostics);

                            return Err(diagnostics);
                        }
                    }

                    let local_date_time_schema = match value_schema {
                        schema_store::ValueSchema::LocalDateTime(local_date_time_schema) => {
                            local_date_time_schema
                        }
                        schema_store::ValueSchema::OneOf(one_of_schema) => {
                            return validate_one_of(
                                self,
                                accessors,
                                one_of_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await
                        }
                        schema_store::ValueSchema::AnyOf(any_of_schema) => {
                            return validate_any_of(
                                self,
                                accessors,
                                any_of_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await
                        }
                        schema_store::ValueSchema::AllOf(all_of_schema) => {
                            return validate_all_of(
                                self,
                                accessors,
                                all_of_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await
                        }
                        _ => unreachable!("Expected a LocalDateTime schema"),
                    };

                    let value_string = self.node().to_string();
                    if let Some(enumerate) = &local_date_time_schema.enumerate {
                        if !enumerate.contains(&value_string) {
                            crate::Error {
                                kind: crate::ErrorKind::Eunmerate {
                                    expected: enumerate.iter().map(ToString::to_string).collect(),
                                    actual: value_string,
                                },
                                range: self.range(),
                            }
                            .set_diagnostics(&mut diagnostics);
                        }
                    }
                }
                _ => unreachable!("Expected a LocalDateTime schema"),
            }

            if diagnostics.is_empty() {
                Ok(())
            } else {
                Err(diagnostics)
            }
        }
        .boxed()
    }
}
