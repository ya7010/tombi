use config::TomlVersion;
use schema_store::{Accessor, ValueSchema, ValueType};

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
            value_type => {
                return Err(vec![crate::Error {
                    kind: crate::ErrorKind::TypeMismatch {
                        expected: schema_store::ValueType::Table,
                        actual: value_type,
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
            _ => unreachable!("Expected a table schema"),
        };

        for (key, value) in self.key_values() {
            let accessor = Accessor::Key(key.to_raw_text(toml_version));
            if table_schema.additional_properties == false
                && table_schema.properties.get(&accessor).is_none()
            {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::KeyNotAllowed {
                        key: key.to_string(),
                    },
                    range: key.range() + value.range(),
                });
                continue;
            }

            if let Some(mut property) = table_schema.properties.get_mut(&accessor) {
                if let Ok(value_schema) = property.resolve(definitions) {
                    if let Err(errs) = value.validate(toml_version, value_schema, &definitions) {
                        errors.extend(errs);
                    }
                }
            } else if let Some(additional_property_schema) =
                &table_schema.additional_property_schema
            {
                if let Ok(mut additional_property_schema) = additional_property_schema.write() {
                    if let Ok(value_schema) = additional_property_schema.resolve(definitions) {
                        if let Err(errs) = value.validate(toml_version, value_schema, &definitions)
                        {
                            errors.extend(errs);
                        }
                    }
                }
            };
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
