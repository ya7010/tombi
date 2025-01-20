use document_tree::ValueImpl;
use regex::Regex;
use schema_store::ValueType;

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for document_tree::String {
    fn validate(
        &self,
        toml_version: config::TomlVersion,
        value_schema: &schema_store::ValueSchema,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Result<(), Vec<crate::Error>> {
        let mut errors = vec![];

        match value_schema.value_type() {
            ValueType::String | ValueType::OneOf(_) | ValueType::AnyOf(_) | ValueType::AllOf(_) => {
            }
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

        let string_schema = match value_schema {
            schema_store::ValueSchema::String(string_schema) => string_schema,
            schema_store::ValueSchema::OneOf(one_of_schema) => {
                return validate_one_of(self, toml_version, one_of_schema, definitions)
            }
            schema_store::ValueSchema::AnyOf(any_of_schema) => {
                return validate_any_of(self, toml_version, any_of_schema, definitions)
            }
            schema_store::ValueSchema::AllOf(all_of_schema) => {
                return validate_all_of(self, toml_version, all_of_schema, definitions)
            }
            _ => unreachable!("Expected a String schema"),
        };

        let value = self.to_raw_string(toml_version);
        if let Some(enumerate) = &string_schema.enumerate {
            if !enumerate.contains(&value) {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::Eunmerate {
                        expected: enumerate.iter().map(|s| format!("\"{s}\"")).collect(),
                        actual: self.value().to_string(),
                    },
                    range: self.range(),
                });
            }
        }

        if let Some(max_length) = &string_schema.max_length {
            if value.len() > *max_length {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::MaximumLength {
                        maximum: *max_length,
                        actual: value.len(),
                    },
                    range: self.range(),
                });
            }
        }

        if let Some(min_length) = &string_schema.min_length {
            if value.len() < *min_length {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::MinimumLength {
                        minimum: *min_length,
                        actual: value.len(),
                    },
                    range: self.range(),
                });
            }
        }

        if let Some(pattern) = &string_schema.pattern {
            if let Ok(regex) = Regex::new(pattern) {
                if !regex.is_match(&value) {
                    errors.push(crate::Error {
                        kind: crate::ErrorKind::Pattern {
                            pattern: pattern.clone(),
                            actual: value,
                        },
                        range: self.range(),
                    });
                }
            } else {
                tracing::error!("Invalid regex pattern: {:?}", pattern);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
