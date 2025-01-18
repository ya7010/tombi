use schema_store::{Accessor, ValueSchema};

use super::Validate;

impl Validate for document_tree::Table {
    fn validate(
        &self,
        errors: &mut Vec<crate::Error>,
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
                if !table_schema.additional_properties {
                    errors.push(crate::Error {
                        kind: crate::ErrorKind::KeyNotAllowed {
                            key: key.to_string(),
                        },
                        range: key.range() + value.range(),
                    });
                }

                value.validate(errors, &value_schema, &definitions);
            }
        }
    }
}
