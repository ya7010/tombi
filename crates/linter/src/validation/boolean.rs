use config::TomlVersion;
use itertools::Itertools;
use schema_store::{SchemaDefinitions, ValueSchema, ValueType};

use super::Validate;

impl Validate for document_tree::Boolean {
    fn validate(
        &self,
        _toml_version: TomlVersion,
        value_schema: &ValueSchema,
        _definitions: &SchemaDefinitions,
    ) -> Result<(), Vec<crate::Error>> {
        let mut errors = vec![];

        match value_schema.value_type() {
            ValueType::Boolean => {}
            value_type => {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::TypeMismatch {
                        expected: ValueType::Boolean,
                        actual: value_type,
                    },
                    range: self.range(),
                });
            }
        }
        let boolean_schema = match value_schema {
            ValueSchema::Boolean(boolean_schema) => boolean_schema,
            _ => unreachable!("Expected a boolean schema"),
        };

        if let Some(enumerate) = &boolean_schema.enumerate {
            let value = self.value();
            if !enumerate.contains(&value) {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::InvalidValue {
                        expected: format!(
                            "[{}]",
                            enumerate.into_iter().map(ToString::to_string).join(", ")
                        ),
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
}
