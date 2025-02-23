mod array;
mod boolean;
mod float;
mod integer;
mod local_date;
mod local_date_time;
mod local_time;
mod offset_date_time;
mod string;
mod table;

use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, SchemaUrl, ValueSchema};

use super::{GetHoverContent, HoverContent};

impl GetHoverContent for document_tree::Value {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [Accessor],
        value_schema: Option<&'a ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: &'a schema_store::SchemaDefinitions,
        sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        async move {
            match self {
                Self::Boolean(boolean) => {
                    boolean
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Integer(integer) => {
                    integer
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Float(float) => {
                    float
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::String(string) => {
                    string
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::OffsetDateTime(offset_date_time) => {
                    offset_date_time
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::LocalDateTime(local_date_time) => {
                    local_date_time
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::LocalDate(local_date) => {
                    local_date
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::LocalTime(local_time) => {
                    local_time
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Array(array) => {
                    array
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Table(table) => {
                    table
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Incomplete { range } => match value_schema {
                    Some(value_schema) => value_schema
                        .get_hover_content(
                            accessors,
                            Some(value_schema),
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                        .map(|mut hover_content| {
                            hover_content.range = Some(*range);
                            hover_content
                        }),
                    None => None,
                },
            }
        }
        .boxed()
    }
}

impl GetHoverContent for ValueSchema {
    fn get_hover_content<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [Accessor],
        value_schema: Option<&'a ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: &'a schema_store::SchemaDefinitions,
        sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Option<HoverContent>> {
        async move {
            match self {
                Self::Boolean(boolean_schema) => {
                    boolean_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Integer(integer_schema) => {
                    integer_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Float(float_schema) => {
                    float_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::String(string_schema) => {
                    string_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::OffsetDateTime(offset_date_time_schema) => {
                    offset_date_time_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::LocalDateTime(local_date_time_schema) => {
                    local_date_time_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::LocalDate(local_date_schema) => {
                    local_date_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::LocalTime(local_time_schema) => {
                    local_time_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Array(array_schema) => {
                    array_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Table(table_schema) => {
                    table_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::OneOf(one_of_schema) => {
                    one_of_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::AnyOf(any_of_schema) => {
                    any_of_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::AllOf(all_of_schema) => {
                    all_of_schema
                        .get_hover_content(
                            accessors,
                            value_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            sub_schema_url_map,
                            schema_store,
                        )
                        .await
                }
                Self::Null => None,
            }
        }
        .boxed()
    }
}
