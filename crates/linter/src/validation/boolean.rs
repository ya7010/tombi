use config::TomlVersion;
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
        let _boolean_schema = match value_schema {
            ValueSchema::Boolean(boolean_schema) => boolean_schema,
            _ => unreachable!("Expected a boolean schema"),
        };

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
