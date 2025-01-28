use config::TomlVersion;
use document_tree::ValueImpl;
use schema_store::{Accessor, ValueSchema, ValueType};

use crate::error::Patterns;

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for document_tree::Table {
    fn validate(
        &self,
        toml_version: TomlVersion,
        value_schema: &ValueSchema,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Result<(), Vec<crate::Error>> {
        let mut errors = vec![];
        match value_schema.value_type() {
            ValueType::Table | ValueType::OneOf(_) | ValueType::AnyOf(_) | ValueType::AllOf(_) => {}
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
                return validate_one_of(self, toml_version, one_of_schema, definitions)
            }
            ValueSchema::AnyOf(any_of_schema) => {
                return validate_any_of(self, toml_version, any_of_schema, definitions)
            }
            ValueSchema::AllOf(all_of_schema) => {
                return validate_all_of(self, toml_version, all_of_schema, definitions)
            }
            _ => unreachable!("Expected a Table schema"),
        };

        for (key, value) in self.key_values() {
            let accessor_raw_text = key.to_raw_text(toml_version);
            let accessor = Accessor::Key(accessor_raw_text.clone());

            let mut matche_key = false;
            if let Some(mut property) = table_schema.properties.get_mut(&accessor) {
                matche_key = true;
                if let Ok(value_schema) = property.resolve(definitions) {
                    if let Err(errs) = value.validate(toml_version, value_schema, definitions) {
                        errors.extend(errs);
                    }
                }
            }

            if let Some(pattern_properties) = &table_schema.pattern_properties {
                for mut pattern_property in pattern_properties.iter_mut() {
                    let property_key = pattern_property.key();
                    let Ok(pattern) = regex::Regex::new(property_key) else {
                        tracing::error!("Invalid regex pattern property: {}", property_key);
                        continue;
                    };
                    if pattern.is_match(&accessor_raw_text) {
                        matche_key = true;
                        let property_schema = pattern_property.value_mut();
                        if let Ok(value_schema) = property_schema.resolve(definitions) {
                            if let Err(errs) =
                                value.validate(toml_version, value_schema, definitions)
                            {
                                errors.extend(errs);
                            }
                        }
                    }
                }
            }
            if !matche_key {
                if let Some(additional_property_schema) = &table_schema.additional_property_schema {
                    if let Ok(mut additional_property_schema) = additional_property_schema.write() {
                        if let Ok(value_schema) = additional_property_schema.resolve(definitions) {
                            if let Err(errs) =
                                value.validate(toml_version, value_schema, definitions)
                            {
                                errors.extend(errs);
                            }
                        }
                    }
                    continue;
                }
                if let Some(pattern_properties) = &table_schema.pattern_properties {
                    errors.push(crate::Error {
                        kind: crate::ErrorKind::PatternProperty {
                            patterns: Patterns(
                                pattern_properties
                                    .iter()
                                    .map(|p| p.key().to_string())
                                    .collect(),
                            ),
                        },
                        range: key.range(),
                    });
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
}
