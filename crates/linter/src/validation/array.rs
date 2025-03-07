use std::borrow::Cow;

use diagnostic::SetDiagnostics;
use document_tree::ValueImpl;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{CurrentSchema, ValueSchema, ValueType};

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for document_tree::Array {
    fn validate<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [schema_store::SchemaAccessor],
        value_schema: Option<&'a schema_store::ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a schema_store::SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext,
    ) -> BoxFuture<'b, Result<(), Vec<diagnostic::Diagnostic>>> {
        async move {
            if let Some(sub_schema_url) = schema_context
                .sub_schema_url_map
                .and_then(|map| map.get(accessors))
            {
                if schema_url != Some(sub_schema_url) {
                    if let Ok(Some(document_schema)) = schema_context
                        .store
                        .try_get_document_schema(sub_schema_url)
                        .await
                    {
                        return self
                            .validate(
                                accessors,
                                document_schema.value_schema.as_ref(),
                                Some(&document_schema.schema_url),
                                Some(&document_schema.definitions),
                                schema_context,
                            )
                            .await;
                    }
                }
            }

            let mut diagnostics = vec![];
            match (value_schema, schema_url, definitions) {
                (Some(value_schema), Some(schema_url), Some(definitions)) => {
                    match value_schema.value_type().await {
                        ValueType::Array
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

                    let array_schema = match value_schema {
                        ValueSchema::Array(array_schema) => array_schema,
                        ValueSchema::OneOf(one_of_schema) => {
                            return validate_one_of(
                                self,
                                accessors,
                                one_of_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await;
                        }
                        ValueSchema::AnyOf(any_of_schema) => {
                            return validate_any_of(
                                self,
                                accessors,
                                any_of_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await;
                        }
                        ValueSchema::AllOf(all_of_schema) => {
                            return validate_all_of(
                                self,
                                accessors,
                                all_of_schema,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await;
                        }
                        _ => unreachable!("Expected an Array schema"),
                    };

                    if let Some(items) = &array_schema.items {
                        let mut referable_schema = items.write().await;
                        if let Ok(Some(CurrentSchema {
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
                        {
                            if value_schema.deprecated() == Some(true) {
                                crate::Warning {
                                    kind: crate::WarningKind::Deprecated,
                                    range: self.range(),
                                }
                                .set_diagnostics(&mut diagnostics);
                            }

                            for value in self.values().iter() {
                                if let Err(schema_diagnostics) = value
                                    .validate(
                                        &accessors
                                            .iter()
                                            .cloned()
                                            .chain(std::iter::once(
                                                schema_store::SchemaAccessor::Index,
                                            ))
                                            .collect::<Vec<_>>(),
                                        Some(value_schema),
                                        Some(&schema_url),
                                        Some(&definitions),
                                        schema_context,
                                    )
                                    .await
                                {
                                    diagnostics.extend(schema_diagnostics);
                                }
                            }
                        }
                    }

                    if let Some(max_items) = array_schema.max_items {
                        if self.values().len() > max_items {
                            crate::Error {
                                kind: crate::ErrorKind::MaxItems {
                                    max_items,
                                    actual: self.values().len(),
                                },
                                range: self.range(),
                            }
                            .set_diagnostics(&mut diagnostics);
                        }
                    }

                    if let Some(min_items) = array_schema.min_items {
                        if self.values().len() < min_items {
                            crate::Error {
                                kind: crate::ErrorKind::MinItems {
                                    min_items,
                                    actual: self.values().len(),
                                },
                                range: self.range(),
                            }
                            .set_diagnostics(&mut diagnostics);
                        }
                    }
                }
                _ => {
                    for value in self.values().iter() {
                        if let Err(value_diagnostics) = value
                            .validate(
                                &accessors
                                    .iter()
                                    .cloned()
                                    .chain(std::iter::once(schema_store::SchemaAccessor::Index))
                                    .collect::<Vec<_>>(),
                                None,
                                schema_url,
                                definitions,
                                schema_context,
                            )
                            .await
                        {
                            diagnostics.extend(value_diagnostics);
                        }
                    }
                }
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
