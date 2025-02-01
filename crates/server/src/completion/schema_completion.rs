use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::Url;

use super::{
    value::{
        find_all_if_completion_items, find_any_of_completion_items, find_one_of_completion_items,
    },
    CompletionContent, CompletionHint, FindCompletionContents,
};

/// A tag data that indicates that only schema information is used for completion.
pub struct SchemaCompletion;

impl FindCompletionContents for SchemaCompletion {
    fn find_completion_contents(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: Option<&SchemaDefinitions>,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        let Some(value_schema) = value_schema else {
            unreachable!(
                "SchemaCompletion::find_completion_contents called without a value schema"
            );
        };

        match value_schema {
            ValueSchema::Boolean(boolean_schema) => boolean_schema.find_completion_contents(
                accessors,
                None,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::Integer(integer_schema) => integer_schema.find_completion_contents(
                accessors,
                None,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::Float(float_schema) => float_schema.find_completion_contents(
                accessors,
                None,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::String(string_schema) => string_schema.find_completion_contents(
                accessors,
                None,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::OffsetDateTime(offset_date_time_schema) => offset_date_time_schema
                .find_completion_contents(
                    accessors,
                    None,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    completion_hint,
                ),
            ValueSchema::LocalDateTime(local_date_time_schema) => local_date_time_schema
                .find_completion_contents(
                    accessors,
                    None,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    completion_hint,
                ),
            ValueSchema::LocalDate(local_date_schema) => local_date_schema
                .find_completion_contents(
                    accessors,
                    None,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    completion_hint,
                ),
            ValueSchema::LocalTime(local_time_schema) => local_time_schema
                .find_completion_contents(
                    accessors,
                    None,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    completion_hint,
                ),
            ValueSchema::Array(array_schema) => array_schema.find_completion_contents(
                accessors,
                None,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::Table(table_schema) => table_schema.find_completion_contents(
                accessors,
                None,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::OneOf(one_of_schema) => find_one_of_completion_items(
                self,
                accessors,
                one_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::AnyOf(any_of_schema) => find_any_of_completion_items(
                self,
                accessors,
                any_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::AllOf(all_of_schema) => find_all_if_completion_items(
                self,
                accessors,
                all_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            ValueSchema::Null => Vec::with_capacity(0),
        }
    }
}
