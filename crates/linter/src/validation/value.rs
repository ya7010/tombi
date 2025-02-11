use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{SchemaDefinitions, ValueSchema};

use super::Validate;

impl Validate for document_tree::Value {
    fn validate<'a: 'b, 'b>(
        &'a self,
        toml_version: TomlVersion,
        value_schema: &'a ValueSchema,
        definitions: &'a SchemaDefinitions,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Result<(), Vec<crate::Error>>> {
        async move {
            match self {
                Self::Boolean(boolean) => {
                    boolean
                        .validate(toml_version, value_schema, definitions, &schema_store)
                        .await
                }
                Self::Integer(integer) => {
                    integer
                        .validate(toml_version, value_schema, definitions, &schema_store)
                        .await
                }
                Self::Float(float) => {
                    float
                        .validate(toml_version, value_schema, definitions, &schema_store)
                        .await
                }
                Self::String(string) => {
                    string
                        .validate(toml_version, value_schema, definitions, &schema_store)
                        .await
                }
                Self::OffsetDateTime(offset_date_time) => {
                    offset_date_time
                        .validate(toml_version, value_schema, definitions, &schema_store)
                        .await
                }
                Self::LocalDateTime(local_date_time) => {
                    local_date_time
                        .validate(toml_version, value_schema, definitions, &schema_store)
                        .await
                }
                Self::LocalDate(local_date) => {
                    local_date
                        .validate(toml_version, value_schema, definitions, schema_store)
                        .await
                }
                Self::LocalTime(local_time) => {
                    local_time
                        .validate(toml_version, value_schema, definitions, schema_store)
                        .await
                }
                Self::Array(array) => {
                    array
                        .validate(toml_version, value_schema, definitions, &schema_store)
                        .await
                }
                Self::Table(table) => {
                    table
                        .validate(toml_version, value_schema, definitions, schema_store)
                        .await
                }
                Self::Incomplete { .. } => Ok(()),
            }
        }
        .boxed()
    }
}
