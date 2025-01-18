use config::TomlVersion;
use schema_store::{Accessor, ValueSchema};

use super::Validate;

impl Validate for document_tree::Table {
    fn validate(
        &self,
        errors: &mut Vec<crate::Error>,
        toml_version: TomlVersion,
        value_schema: &ValueSchema,
        definitions: &schema_store::SchemaDefinitions,
    ) {
        let table_schema = match value_schema {
            ValueSchema::Table(table_schema) => table_schema,
            _ => return,
        };

        for (key, value) in self.key_values() {
            if table_schema.additional_properties == false
                && table_schema
                    .properties
                    .get(&Accessor::Key(key.to_string()))
                    .is_none()
            {
                let accessor = Accessor::Key(key.to_raw_text(toml_version));

                if !table_schema.additional_properties {
                    errors.push(crate::Error {
                        kind: crate::ErrorKind::KeyNotAllowed {
                            key: key.to_string(),
                        },
                        range: key.range() + value.range(),
                    });
                }
                if let Some(mut property) = table_schema.properties.get_mut(&accessor) {
                    if let Ok(value_schema) = property.resolve(definitions) {
                        value.validate(errors, toml_version, value_schema, &definitions)
                    }
                } else if let Some(additional_property_schema) =
                    &table_schema.additional_property_schema
                {
                    if let Ok(mut additional_property_schema) = additional_property_schema.write() {
                        if let Ok(value_schema) = additional_property_schema.resolve(definitions) {
                            value.validate(errors, toml_version, value_schema, &definitions)
                        }
                    }
                };
            }
        }
    }
}
