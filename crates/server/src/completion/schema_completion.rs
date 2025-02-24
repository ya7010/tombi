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
        position: text::Position,
        keys: &'a [document_tree::Key],
        accessors: &'a [Accessor],
        schema_url: Option<&'a SchemaUrl>,
        value_schema: Option<&'a ValueSchema>,
        definitions: Option<&'a SchemaDefinitions>,
        schema_context: &'a schema_store::SchemaContext<'a>,
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
                            position,
                            keys,
                            accessors,
                            schema_url,
                            None,
                            definitions,
                            schema_context,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::Integer(integer_schema) => {
                    integer_schema
                        .find_completion_contents(
                            position,
                            keys,
                            accessors,
                            schema_url,
                            None,
                            definitions,
                            schema_context,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::Float(float_schema) => {
                    float_schema
                        .find_completion_contents(
                            position,
                            keys,
                            accessors,
                            schema_url,
                            None,
                            definitions,
                            schema_context,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::String(string_schema) => {
                    string_schema
                        .find_completion_contents(
                            position,
                            keys,
                            accessors,
                            schema_url,
                            None,
                            definitions,
                            schema_context,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::OffsetDateTime(offset_date_time_schema) => {
                    offset_date_time_schema
                        .find_completion_contents(
                            position,
                            keys,
                            accessors,
                            schema_url,
                            None,
                            definitions,
                            schema_context,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::LocalDateTime(local_date_time_schema) => {
                    local_date_time_schema
                        .find_completion_contents(
                            position,
                            keys,
                            accessors,
                            schema_url,
                            None,
                            definitions,
                            schema_context,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::LocalDate(local_date_schema) => {
                    local_date_schema
                        .find_completion_contents(
                            position,
                            keys,
                            accessors,
                            schema_url,
                            None,
                            definitions,
                            schema_context,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::LocalTime(local_time_schema) => {
                    local_time_schema
                        .find_completion_contents(
                            position,
                            keys,
                            accessors,
                            schema_url,
                            None,
                            definitions,
                            schema_context,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::Array(array_schema) => {
                    array_schema
                        .find_completion_contents(
                            position,
                            keys,
                            accessors,
                            schema_url,
                            None,
                            definitions,
                            schema_context,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::Table(table_schema) => {
                    table_schema
                        .find_completion_contents(
                            position,
                            keys,
                            accessors,
                            schema_url,
                            None,
                            definitions,
                            schema_context,
                            completion_hint,
                        )
                        .await
                }
                ValueSchema::OneOf(one_of_schema) => {
                    find_one_of_completion_items(
                        self,
                        position,
                        keys,
                        accessors,
                        schema_url,
                        one_of_schema,
                        definitions,
                        schema_context,
                        completion_hint,
                    )
                    .await
                }
                ValueSchema::AnyOf(any_of_schema) => {
                    find_any_of_completion_items(
                        self,
                        position,
                        keys,
                        accessors,
                        schema_url,
                        any_of_schema,
                        definitions,
                        schema_context,
                        completion_hint,
                    )
                    .await
                }
                ValueSchema::AllOf(all_of_schema) => {
                    find_all_of_completion_items(
                        self,
                        position,
                        keys,
                        accessors,
                        schema_url,
                        all_of_schema,
                        definitions,
                        schema_context,
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

impl linter::Validate for SchemaCompletion {
    fn validate<'a: 'b, 'b>(
        &'a self,
        _toml_version: TomlVersion,
        _accessors: &'a [Accessor],
        _value_schema: Option<&'a ValueSchema>,
        _schema_url: Option<&'a schema_store::SchemaUrl>,
        _definitions: Option<&'a SchemaDefinitions>,
        _sub_schema_url_map: &'a schema_store::SubSchemaUrlMap,
        _schema_store: &'a schema_store::SchemaStore,
    ) -> BoxFuture<'b, Result<(), Vec<linter::Error>>> {
        async move { Ok(()) }.boxed()
    }
}
