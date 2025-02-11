use config::TomlVersion;
use futures::{future::BoxFuture, FutureExt};
use schema_store::{Accessor, SchemaDefinitions, SchemaStore, SchemaUrl, ValueSchema};

use super::{
    value::{
        find_all_of_completion_items, find_any_of_completion_items, find_one_of_completion_items,
    },
    CompletionContent, CompletionHint, FindCompletionContents,
};

/// A tag data that indicates that only schema information is used for completion.
pub struct SchemaCompletion;

impl FindCompletionContents for SchemaCompletion {
    fn find_completion_contents<'a: 'b, 'b>(
        &'a self,
        accessors: &'a Vec<Accessor>,
        value_schema: Option<&'a ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &'a [document_tree::Key],
        schema_url: Option<&'a SchemaUrl>,
        definitions: Option<&'a SchemaDefinitions>,
        schema_store: &'a SchemaStore,
        completion_hint: Option<CompletionHint>,
    ) -> BoxFuture<'b, Vec<CompletionContent>> {
        tracing::trace!("accessors: {:?}", accessors);
        tracing::trace!("keys: {:?}", keys);
        tracing::trace!("value_schema: {:?}", value_schema);
        tracing::trace!("completion_hint: {:?}", completion_hint);

        async move {
            let Some(value_schema) = value_schema else {
                unreachable!(
                    "SchemaCompletion::find_completion_contents called without a value schema"
                );
            };

            match value_schema {
                ValueSchema::Boolean(boolean_schema) => {
                    boolean_schema
                        .find_completion_contents(
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            schema_store,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::Integer(integer_schema) => {
                    integer_schema
                        .find_completion_contents(
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            schema_store,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::Float(float_schema) => {
                    float_schema
                        .find_completion_contents(
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            schema_store,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::String(string_schema) => {
                    string_schema
                        .find_completion_contents(
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            schema_store,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::OffsetDateTime(offset_date_time_schema) => {
                    offset_date_time_schema
                        .find_completion_contents(
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            schema_store,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::LocalDateTime(local_date_time_schema) => {
                    local_date_time_schema
                        .find_completion_contents(
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            schema_store,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::LocalDate(local_date_schema) => {
                    local_date_schema
                        .find_completion_contents(
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            schema_store,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::LocalTime(local_time_schema) => {
                    local_time_schema
                        .find_completion_contents(
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            schema_store,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::Array(array_schema) => {
                    array_schema
                        .find_completion_contents(
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            schema_store,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::Table(table_schema) => {
                    table_schema
                        .find_completion_contents(
                            accessors,
                            None,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                            schema_store,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::OneOf(one_of_schema) => {
                    find_one_of_completion_items(
                        self,
                        accessors,
                        one_of_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        schema_store,
                        completion_hint,
                    )
                    .await
                }
                ValueSchema::AnyOf(any_of_schema) => {
                    find_any_of_completion_items(
                        self,
                        accessors,
                        any_of_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        schema_store,
                        completion_hint,
                    )
                    .await
                }
                ValueSchema::AllOf(all_of_schema) => {
                    find_all_of_completion_items(
                        self,
                        accessors,
                        all_of_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        schema_store,
                        completion_hint,
                    )
                    .await
                }
                ValueSchema::Null => Vec::with_capacity(0),
            }
        }
        .boxed()
    }
}
