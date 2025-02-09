use config::TomlVersion;
use document_tree::ValueImpl;
use futures::future::BoxFuture;
use futures::FutureExt;
use schema_store::{SchemaDefinitions, ValueSchema, ValueType};

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for document_tree::Array {
    fn validate<'a: 'b, 'b>(
        &'a self,
        toml_version: TomlVersion,
        value_schema: &'a ValueSchema,
        definitions: &'a SchemaDefinitions,
    ) -> BoxFuture<'b, Result<(), Vec<crate::Error>>> {
        async move {
            match value_schema.value_type() {
                ValueType::Array
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

            let array_schema = match value_schema {
                ValueSchema::Array(array_schema) => array_schema,
                ValueSchema::OneOf(one_of_schema) => {
                    return validate_one_of(self, toml_version, one_of_schema, definitions).await
                }
                ValueSchema::AnyOf(any_of_schema) => {
                    return validate_any_of(self, toml_version, any_of_schema, definitions).await
                }
                ValueSchema::AllOf(all_of_schema) => {
                    return validate_all_of(self, toml_version, all_of_schema, definitions).await
                }
                _ => unreachable!("Expected an Array schema"),
            };

            let mut errors = vec![];
            if let Some(items) = &array_schema.items_tokio {
                let mut referable_schema = items.write().await;
                if let Ok(item_schema) = referable_schema.resolve(definitions) {
                    for value in self.values() {
                        if let Err(errs) =
                            value.validate(toml_version, item_schema, definitions).await
                        {
                            errors.extend(errs);
                        }
                    }
                }
            }

            if let Some(max_items) = array_schema.max_items {
                if self.values().len() > max_items {
                    errors.push(crate::Error {
                        kind: crate::ErrorKind::MaxItems {
                            max_items,
                            actual: self.values().len(),
                        },
                        range: self.range(),
                    });
                }
            }

            if let Some(min_items) = array_schema.min_items {
                if self.values().len() < min_items {
                    errors.push(crate::Error {
                        kind: crate::ErrorKind::MinItems {
                            min_items,
                            actual: self.values().len(),
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
