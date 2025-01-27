use config::TomlVersion;
use document_tree::ValueImpl;
use schema_store::{ValueSchema, ValueType};

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for document_tree::Array {
    fn validate(
        &self,
        toml_version: TomlVersion,
        value_schema: &ValueSchema,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Result<(), Vec<crate::Error>> {
        match value_schema.value_type() {
            ValueType::Array | ValueType::OneOf(_) | ValueType::AnyOf(_) | ValueType::AllOf(_) => {}
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
                return validate_one_of(self, toml_version, one_of_schema, definitions)
            }
            ValueSchema::AnyOf(any_of_schema) => {
                return validate_any_of(self, toml_version, any_of_schema, definitions)
            }
            ValueSchema::AllOf(all_of_schema) => {
                return validate_all_of(self, toml_version, all_of_schema, definitions)
            }
            _ => unreachable!("Expected an Array schema"),
        };

        let mut errors = vec![];
        for value in self.values() {
            array_schema.operate_item(
                |item_schema| {
                    if let Err(errs) = value.validate(toml_version, item_schema, definitions) {
                        errors.extend(errs);
                    }
                },
                definitions,
            );
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
}
