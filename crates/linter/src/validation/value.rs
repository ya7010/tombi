use config::TomlVersion;
use schema_store::{SchemaDefinitions, ValueSchema};

use super::Validate;

impl Validate for document_tree::Value {
    fn validate(
        &self,
        toml_version: TomlVersion,
        value_schema: &ValueSchema,
        definitions: &SchemaDefinitions,
    ) -> Result<(), Vec<crate::Error>> {
        match self {
            Self::Boolean(boolean) => boolean.validate(toml_version, value_schema, definitions),
            Self::Integer(integer) => integer.validate(toml_version, value_schema, definitions),
            Self::Float(float) => float.validate(toml_version, value_schema, definitions),
            Self::String(string) => string.validate(toml_version, value_schema, definitions),
            Self::OffsetDateTime(offset_date_time) => {
                offset_date_time.validate(toml_version, value_schema, definitions)
            }
            Self::LocalDateTime(local_date_time) => {
                local_date_time.validate(toml_version, value_schema, definitions)
            }
            Self::LocalDate(local_date) => {
                local_date.validate(toml_version, value_schema, definitions)
            }
            Self::LocalTime(local_time) => {
                local_time.validate(toml_version, value_schema, definitions)
            }
            Self::Array(array) => array.validate(toml_version, value_schema, definitions),
            Self::Table(table) => table.validate(toml_version, value_schema, definitions),
            Self::Incomplete { .. } => Ok(()),
        }
    }
}
