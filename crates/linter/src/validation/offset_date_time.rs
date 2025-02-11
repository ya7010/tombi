use config::TomlVersion;
use document_tree::{OffsetDateTime, ValueImpl};
use futures::{future::BoxFuture, FutureExt};
use schema_store::{SchemaDefinitions, ValueSchema, ValueType};

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for OffsetDateTime {
    fn validate<'a: 'b, 'b>(
        &'a self,
        toml_version: TomlVersion,
        value_schema: &'a ValueSchema,
        definitions: &'a SchemaDefinitions,
    ) -> BoxFuture<'b, Result<(), Vec<crate::Error>>> {
        async move {
            let mut errors = vec![];

            match value_schema.value_type().await {
                ValueType::OffsetDateTime
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

            let offset_date_time_schema = match value_schema {
                schema_store::ValueSchema::OffsetDateTime(offset_date_time_schema) => {
                    offset_date_time_schema
                }
                schema_store::ValueSchema::OneOf(one_of_schema) => {
                    return validate_one_of(self, toml_version, one_of_schema, definitions).await
                }
                schema_store::ValueSchema::AnyOf(any_of_schema) => {
                    return validate_any_of(self, toml_version, any_of_schema, definitions).await
                }
                schema_store::ValueSchema::AllOf(all_of_schema) => {
                    return validate_all_of(self, toml_version, all_of_schema, definitions).await
                }
                _ => unreachable!("Expected an Offset Date-Time schema"),
            };

            let value_string = self.node().to_string();
            if let Some(enumerate) = &offset_date_time_schema.enumerate {
                if !enumerate.contains(&value_string) {
                    errors.push(crate::Error {
                        kind: crate::ErrorKind::Eunmerate {
                            expected: enumerate.iter().map(ToString::to_string).collect(),
                            actual: value_string,
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
