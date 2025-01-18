use document_tree::ValueImpl;
use schema_store::ValueType;

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for document_tree::Integer {
    fn validate(
        &self,
        toml_version: config::TomlVersion,
        value_schema: &schema_store::ValueSchema,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Result<(), Vec<crate::Error>> {
        let mut errors = vec![];

        match value_schema.value_type() {
            ValueType::Integer
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

        let integer_schema = match value_schema {
            schema_store::ValueSchema::Integer(integer_schema) => integer_schema,
            schema_store::ValueSchema::OneOf(one_of_schema) => {
                return validate_one_of(self, toml_version, one_of_schema, definitions)
            }
            schema_store::ValueSchema::AnyOf(any_of_schema) => {
                return validate_any_of(self, toml_version, any_of_schema, definitions)
            }
            schema_store::ValueSchema::AllOf(all_of_schema) => {
                return validate_all_of(self, toml_version, all_of_schema, definitions)
            }
            _ => unreachable!("Expected an Integer schema"),
        };

        let value = self.value();
        if let Some(enumerate) = &integer_schema.enumerate {
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

        if let Some(maximum) = &integer_schema.maximum {
            if value > *maximum {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::MaximumInteger {
                        maximum: *maximum,
                        actual: value,
                    },
                    range: self.range(),
                });
            }
        }

        if let Some(minimum) = &integer_schema.minimum {
            if value < *minimum {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::MinimumInteger {
                        minimum: *minimum,
                        actual: value,
                    },
                    range: self.range(),
                });
            }
        }

        if let Some(exclusive_maximum) = &integer_schema.exclusive_maximum {
            if value >= *exclusive_maximum {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::ExclusiveMaximumInteger {
                        maximum: *exclusive_maximum - 1,
                        actual: value,
                    },
                    range: self.range(),
                });
            }
        }

        if let Some(exclusive_minimum) = &integer_schema.exclusive_minimum {
            if value <= *exclusive_minimum {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::ExclusiveMinimumInteger {
                        minimum: *exclusive_minimum + 1,
                        actual: value,
                    },
                    range: self.range(),
                });
            }
        }

        if let Some(multiple_of) = &integer_schema.multiple_of {
            if value % *multiple_of != 0 {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::MultipleOfInteger {
                        multiple_of: *multiple_of,
                        actual: value,
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
