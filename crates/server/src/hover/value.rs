use schema_store::ValueSchema;
use tower_lsp::lsp_types::Url;

use super::GetHoverContent;

impl GetHoverContent for document_tree::Value {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        value_schema: Option<&schema_store::ValueSchema>,
        toml_version: config::TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<super::HoverContent> {
        match self {
            Self::Boolean(boolean) => boolean.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::Integer(integer) => integer.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::Float(float) => float.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::String(string) => string.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::OffsetDateTime(offset_date_time) => offset_date_time.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::LocalDateTime(local_date_time) => local_date_time.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::LocalDate(local_date) => local_date.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::LocalTime(local_time) => local_time.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::Array(array) => array.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::Table(table) => table.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::Incomplete { range } => value_schema.and_then(|schema| {
                schema
                    .get_hover_content(
                        accessors,
                        value_schema,
                        toml_version,
                        position,
                        keys,
                        schema_url,
                        definitions,
                    )
                    .map(|mut hover_content| {
                        hover_content.range = Some(*range);
                        hover_content
                    })
            }),
        }
    }
}

impl GetHoverContent for ValueSchema {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: config::TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<super::HoverContent> {
        match self {
            Self::Boolean(boolean_schema) => boolean_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::Integer(integer_schema) => integer_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::Float(float_schema) => float_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::String(string_schema) => string_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::OffsetDateTime(offset_date_time_schema) => offset_date_time_schema
                .get_hover_content(
                    accessors,
                    value_schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                ),
            Self::LocalDateTime(local_date_time_schema) => local_date_time_schema
                .get_hover_content(
                    accessors,
                    value_schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                ),
            Self::LocalDate(local_date_schema) => local_date_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::LocalTime(local_time_schema) => local_time_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::Array(array_schema) => array_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::Table(table_schema) => table_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::OneOf(one_of_schema) => one_of_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::AnyOf(any_of_schema) => any_of_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::AllOf(all_of_schema) => all_of_schema.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Self::Null => None,
        }
    }
}
