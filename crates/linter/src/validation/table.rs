use config::TomlVersion;
use document_tree::ValueImpl;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, SchemaDefinitions, ValueSchema, ValueType};

use crate::error::Patterns;

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for document_tree::Table {
    fn validate<'a: 'b, 'b>(
        &'a self,
        toml_version: TomlVersion,
        value_schema: &'a ValueSchema,
        definitions: &'a SchemaDefinitions,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Result<(), Vec<crate::Error>>> {
        tracing::trace!("self = {:?}", self);
        tracing::trace!("value_schema = {:?}", value_schema);

        async move {
            let mut errors = vec![];
            match value_schema.value_type().await {
                ValueType::Table
                | ValueType::OneOf(_)
                | ValueType::AnyOf(_)
                | ValueType::AllOf(_) => {}
                ValueType::Null => return Ok(()),
                value_schema => {
                    return Err(vec![crate::Error {
                        kind: crate::ErrorKind::TypeMismatch {
                            expected: value_schema,
                            actual: self.value_type(),
                        },
                        range: self.range(),
                    }])
                }
            }

            let table_schema = match value_schema {
                ValueSchema::Table(table_schema) => table_schema,
                ValueSchema::OneOf(one_of_schema) => {
                    return validate_one_of(
                        self,
                        toml_version,
                        one_of_schema,
                        definitions,
                        schema_store,
                    )
                    .await
                }
                ValueSchema::AnyOf(any_of_schema) => {
                    return validate_any_of(
                        self,
                        toml_version,
                        any_of_schema,
                        definitions,
                        schema_store,
                    )
                    .await
                }
                ValueSchema::AllOf(all_of_schema) => {
                    return validate_all_of(
                        self,
                        toml_version,
                        all_of_schema,
                        definitions,
                        schema_store,
                    )
                    .await
                }
                _ => unreachable!("Expected a Table schema"),
            };

            for (key, value) in self.key_values() {
                let accessor_raw_text = key.to_raw_text(toml_version);
                let accessor = Accessor::Key(accessor_raw_text.clone());

                let mut matche_key = false;
                if let Some(property) = table_schema.properties.write().await.get_mut(&accessor) {
                    matche_key = true;
                    match property.resolve(definitions, schema_store).await {
                        Ok((value_schema, new_schema)) => {
                            let definitions = if let Some((_, new_definitions)) = &new_schema {
                                new_definitions
                            } else {
                                definitions
                            };
                            if let Err(errs) = value
                                .validate(toml_version, value_schema, definitions, schema_store)
                                .await
                            {
                                errors.extend(errs);
                            }
                        }
                        Err(err) => {
                            tracing::error!("{}", err);
                        }
                    }
                }

                if let Some(pattern_properties) = &table_schema.pattern_properties {
                    for (property_key, refferable_pattern_property) in
                        pattern_properties.write().await.iter_mut()
                    {
                        let Ok(pattern) = regex::Regex::new(property_key) else {
                            tracing::error!("Invalid regex pattern property: {}", property_key);
                            continue;
                        };
                        if pattern.is_match(&accessor_raw_text) {
                            matche_key = true;
                            if let Ok((pattern_property, new_schema)) = refferable_pattern_property
                                .resolve(definitions, schema_store)
                                .await
                            {
                                let definitions = if let Some((_, new_definitions)) = &new_schema {
                                    new_definitions
                                } else {
                                    definitions
                                };

                                if let Err(errs) = value
                                    .validate(
                                        toml_version,
                                        pattern_property,
                                        definitions,
                                        schema_store,
                                    )
                                    .await
                                {
                                    errors.extend(errs);
                                }
                            }
                        } else if !table_schema.additional_properties {
                            errors.push(crate::Error {
                                kind: crate::ErrorKind::PatternProperty {
                                    patterns: Patterns(
                                        pattern_properties
                                            .read()
                                            .await
                                            .keys()
                                            .map(ToString::to_string)
                                            .collect(),
                                    ),
                                },
                                range: key.range(),
                            });
                        }
                    }
                }
                if !matche_key {
                    if let Some(additional_property_schema) =
                        &table_schema.additional_property_schema
                    {
                        let mut referable_schema = additional_property_schema.write().await;
                        if let Ok((value_schema, new_schema)) =
                            referable_schema.resolve(definitions, schema_store).await
                        {
                            let definitions = if let Some((_, new_definitions)) = &new_schema {
                                new_definitions
                            } else {
                                definitions
                            };

                            if let Err(errs) = value
                                .validate(toml_version, value_schema, definitions, schema_store)
                                .await
                            {
                                errors.extend(errs);
                            }
                        }
                        continue;
                    }
                    if !table_schema.additional_properties {
                        errors.push(crate::Error {
                            kind: crate::ErrorKind::KeyNotAllowed {
                                key: key.to_string(),
                            },
                            range: key.range() + value.range(),
                        });
                        continue;
                    }
                }
            }

            if let Some(required) = &table_schema.required {
                let keys = self
                    .keys()
                    .map(|key| key.to_raw_text(toml_version))
                    .collect::<Vec<_>>();

                for required_key in required {
                    if !keys.contains(required_key) {
                        errors.push(crate::Error {
                            kind: crate::ErrorKind::KeyRequired {
                                key: required_key.to_string(),
                            },
                            range: self.range(),
                        });
                    }
                }
            }

            if let Some(max_properties) = table_schema.max_properties {
                if self.keys().count() > max_properties {
                    errors.push(crate::Error {
                        kind: crate::ErrorKind::MaxProperties {
                            max_properties,
                            actual: self.keys().count(),
                        },
                        range: self.range(),
                    });
                }
            }

            if let Some(min_properties) = table_schema.min_properties {
                if self.keys().count() < min_properties {
                    errors.push(crate::Error {
                        kind: crate::ErrorKind::MinProperties {
                            min_properties,
                            actual: self.keys().count(),
                        },
                        range: self.range(),
                    });
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
}
