mod all_of;
mod any_of;
mod array;
mod boolean;
mod float;
mod integer;
mod local_date;
mod local_date_time;
mod local_time;
mod offset_date_time;
mod one_of;
mod string;
mod table;

use super::{
    find_all_if_completion_items, find_any_of_completion_items, find_one_of_completion_items,
    CompletionCandidate, CompletionContent, CompletionHint, FindCompletionContents,
};
use config::TomlVersion;
use schema_store::{
    Accessor, ArraySchema, BooleanSchema, FloatSchema, IntegerSchema, LocalDateSchema,
    LocalDateTimeSchema, LocalTimeSchema, OffsetDateTimeSchema, SchemaDefinitions, StringSchema,
    TableSchema, ValueSchema,
};
use tower_lsp::lsp_types::Url;

impl FindCompletionContents for document_tree::Value {
    fn find_completion_contents(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: &ValueSchema,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        match self {
            Self::Boolean(boolean) => boolean.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            Self::Integer(integer) => integer.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            Self::Float(float) => float.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            Self::String(string) => string.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            Self::OffsetDateTime(offset_date_time) => offset_date_time.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            Self::LocalDateTime(local_date_time) => local_date_time.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            Self::LocalDate(local_date) => local_date.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            Self::LocalTime(local_time) => local_time.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            Self::Array(array) => array.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            Self::Table(table) => table.find_completion_contents(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
                completion_hint,
            ),
            Self::Incomplete { .. } => match value_schema {
                ValueSchema::Boolean(boolean_schema) => boolean_schema.find_completion_contents(
                    accessors,
                    value_schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    completion_hint,
                ),
                ValueSchema::Integer(integer_schema) => integer_schema.find_completion_contents(
                    accessors,
                    value_schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    completion_hint,
                ),
                ValueSchema::Float(float_schema) => float_schema.find_completion_contents(
                    accessors,
                    value_schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    completion_hint,
                ),
                ValueSchema::String(string_schema) => string_schema.find_completion_contents(
                    accessors,
                    value_schema,
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
                        value_schema,
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
                        value_schema,
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
                        value_schema,
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
                        value_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                        completion_hint,
                    ),
                ValueSchema::Array(_) => match completion_hint {
                    Some(CompletionHint::InTableHeader) => Vec::with_capacity(0),
                    _ => vec![CompletionContent {
                        label: "[]".to_string(),
                        kind: Some(tower_lsp::lsp_types::CompletionItemKind::VALUE),
                        schema_url: schema_url.cloned(),
                        ..Default::default()
                    }],
                },
                ValueSchema::Table(table_schema) => table_schema.find_completion_contents(
                    accessors,
                    value_schema,
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
            },
        }
    }
}

impl CompletionCandidate for ValueSchema {
    fn title(
        &self,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Option<String> {
        match self {
            Self::Boolean(BooleanSchema { title, .. })
            | Self::Integer(IntegerSchema { title, .. })
            | Self::Float(FloatSchema { title, .. })
            | Self::String(StringSchema { title, .. })
            | Self::OffsetDateTime(OffsetDateTimeSchema { title, .. })
            | Self::LocalDateTime(LocalDateTimeSchema { title, .. })
            | Self::LocalDate(LocalDateSchema { title, .. })
            | Self::LocalTime(LocalTimeSchema { title, .. })
            | Self::Array(ArraySchema { title, .. })
            | Self::Table(TableSchema { title, .. }) => title.as_deref().map(ToString::to_string),
            Self::OneOf(one_of) => one_of.title(definitions, completion_hint),
            Self::AnyOf(any_of) => any_of.title(definitions, completion_hint),
            Self::AllOf(all_of) => all_of.title(definitions, completion_hint),
            Self::Null => None,
        }
    }

    fn description(
        &self,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Option<String> {
        match self {
            Self::Boolean(BooleanSchema { description, .. })
            | Self::Integer(IntegerSchema { description, .. })
            | Self::Float(FloatSchema { description, .. })
            | Self::String(StringSchema { description, .. })
            | Self::OffsetDateTime(OffsetDateTimeSchema { description, .. })
            | Self::LocalDateTime(LocalDateTimeSchema { description, .. })
            | Self::LocalDate(LocalDateSchema { description, .. })
            | Self::LocalTime(LocalTimeSchema { description, .. })
            | Self::Array(ArraySchema { description, .. })
            | Self::Table(TableSchema { description, .. }) => {
                description.as_deref().map(ToString::to_string)
            }
            Self::OneOf(one_of) => one_of.description(definitions, completion_hint),
            Self::AnyOf(any_of) => any_of.description(definitions, completion_hint),
            Self::AllOf(all_of) => all_of.description(definitions, completion_hint),
            Self::Null => None,
        }
    }
}
