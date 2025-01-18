use super::{validate_all_of, validate_any_of, validate_one_of, Validate};
use document_tree::OffsetDateTime;

impl Validate for OffsetDateTime {
    fn validate(
        &self,
        toml_version: config::TomlVersion,
        value_schema: &schema_store::ValueSchema,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Result<(), Vec<crate::Error>> {
        let mut errors = vec![];

        match value_schema.value_type() {
            schema_store::ValueType::OffsetDateTime
            | schema_store::ValueType::OneOf(_)
            | schema_store::ValueType::AnyOf(_)
            | schema_store::ValueType::AllOf(_) => {}
            value_type => {
                return Err(vec![crate::Error {
                    kind: crate::ErrorKind::TypeMismatch {
                        expected: schema_store::ValueType::OffsetDateTime,
                        actual: value_type,
                    },
                    range: self.range(),
                }]);
            }
        }

        let offset_date_time_schema = match value_schema {
            schema_store::ValueSchema::OffsetDateTime(offset_date_time_schema) => {
                offset_date_time_schema
            }
            schema_store::ValueSchema::OneOf(one_of_schema) => {
                return validate_one_of(self, toml_version, one_of_schema, definitions)
            }
            schema_store::ValueSchema::AnyOf(any_of_schema) => {
                return validate_any_of(self, toml_version, any_of_schema, definitions)
            }
            schema_store::ValueSchema::AllOf(all_of_schema) => {
                return validate_all_of(self, toml_version, all_of_schema, definitions)
            }
            _ => unreachable!("Expected an Offset Date-Time schema"),
        };

        let value_string = self.node().to_string();
        if let Some(enumerate) = &offset_date_time_schema.enumerate {
            if !enumerate.contains(&value_string) {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::Eunmerate {
                        expected: enumerate.iter().map(ToString::to_string).collect(),
                        actual: value_string,
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
