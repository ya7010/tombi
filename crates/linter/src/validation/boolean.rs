use config::TomlVersion;
use document_tree::ValueImpl;
use futures::future::BoxFuture;
use futures::FutureExt;
use schema_store::{SchemaDefinitions, ValueSchema, ValueType};

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for document_tree::Boolean {
    fn validate<'a: 'b, 'b>(
        &'a self,
        toml_version: TomlVersion,
        value_schema: &'a ValueSchema,
        definitions: &'a SchemaDefinitions,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Result<(), Vec<crate::Error>>> {
        async move {
            let mut errors = vec![];

            match value_schema.value_type().await {
                ValueType::Boolean
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
                    }]);
                }
            }
            let boolean_schema = match value_schema {
                ValueSchema::Boolean(boolean_schema) => boolean_schema,
                ValueSchema::OneOf(one_of_schema) => {
                    return validate_one_of(
                        self,
                        toml_version,
                        one_of_schema,
                        definitions,
                        &schema_store,
                    )
                    .await
                }
                ValueSchema::AnyOf(any_of_schema) => {
                    return validate_any_of(
                        self,
                        toml_version,
                        any_of_schema,
                        definitions,
                        &schema_store,
                    )
                    .await
                }
                ValueSchema::AllOf(all_of_schema) => {
                    return validate_all_of(
                        self,
                        toml_version,
                        all_of_schema,
                        definitions,
                        &schema_store,
                    )
                    .await
                }
                _ => unreachable!("Expected a Boolean schema"),
            };

            let value = self.value();
            if let Some(enumerate) = &boolean_schema.enumerate {
                if !enumerate.contains(&value) {
                    errors.push(crate::Error {
                        kind: crate::ErrorKind::Eunmerate {
                            expected: enumerate.iter().map(ToString::to_string).collect(),
                            actual: value.to_string(),
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
