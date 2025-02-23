use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};

use super::Validate;

impl Validate for document_tree::Value {
    fn validate<'a: 'b, 'b>(
        &'a self,
        toml_version: TomlVersion,
        accessors: &'a [Accessor],
        value_schema: Option<&'a ValueSchema>,
        schema_url: Option<&'a schema_store::SchemaUrl>,
        definitions: Option<&'a SchemaDefinitions>,
        sub_schema_url_map: &'a schema_store::SubSchemaUrlMap,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Result<(), Vec<crate::Error>>> {
        tracing::trace!("self = {:?}", self);
        tracing::trace!("value_schema = {:?}", value_schema);

        async move {
            match self {
                Self::Boolean(boolean) => {
                    boolean
                        .validate(
                            toml_version,
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Integer(integer) => {
                    integer
                        .validate(
                            toml_version,
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Float(float) => {
                    float
                        .validate(
                            toml_version,
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::String(string) => {
                    string
                        .validate(
                            toml_version,
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::OffsetDateTime(offset_date_time) => {
                    offset_date_time
                        .validate(
                            toml_version,
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::LocalDateTime(local_date_time) => {
                    local_date_time
                        .validate(
                            toml_version,
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::LocalDate(local_date) => {
                    local_date
                        .validate(
                            toml_version,
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::LocalTime(local_time) => {
                    local_time
                        .validate(
                            toml_version,
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Array(array) => {
                    array
                        .validate(
                            toml_version,
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Table(table) => {
                    table
                        .validate(
                            toml_version,
                            accessors,
                            value_schema,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Incomplete { .. } => Ok(()),
            }
        }
        .boxed()
    }
}
