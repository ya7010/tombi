use itertools::Itertools;
use regex::Regex;

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
            schema_store::ValueType::String
            | schema_store::ValueType::OneOf(_)
            | schema_store::ValueType::AnyOf(_)
            | schema_store::ValueType::AllOf(_) => {}
            value_type => {
                return Err(vec![crate::Error {
                    kind: crate::ErrorKind::TypeMismatch {
                        expected: schema_store::ValueType::String,
                        actual: value_type,
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

        let value = self.value();
        if let Some(enumerate) = &string_schema.enumerate {
            if !enumerate.contains(&value.to_string()) {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::InvalidValue {
                        expected: enumerate.into_iter().map(ToString::to_string).join(", "),
                        actual: value.to_string(),
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
            let regex = Regex::new(pattern).map_err(|e| {
                vec![crate::Error {
                    kind: crate::ErrorKind::InvalidPattern {
                        pattern: pattern.clone(),
                        error: e.to_string(),
                    },
                    range: self.range(),
                }]
            })?;
            if !regex.is_match(value) {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::PatternMismatch {
                        pattern: pattern.clone(),
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
