use std::fmt::Debug;

use schema_store::ValueType;

use super::{GetHoverContent, HoverContent};

impl GetHoverContent for document_tree::Value {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        value_schema: Option<&schema_store::ValueSchema>,
        toml_version: config::TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<super::HoverContent> {
        match self {
            Self::Boolean(boolean) => boolean.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                definitions,
            ),
            Self::Integer(integer) => integer.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                definitions,
            ),
            Self::Float(float) => float.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                definitions,
            ),
            Self::String(string) => string.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                definitions,
            ),
            Self::OffsetDateTime(offset_date_time) => offset_date_time.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                definitions,
            ),
            Self::LocalDateTime(local_date_time) => local_date_time.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                definitions,
            ),
            Self::LocalDate(local_date) => local_date.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                definitions,
            ),
            Self::LocalTime(local_time) => local_time.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                definitions,
            ),
            Self::Array(array) => array.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                definitions,
            ),
            Self::Table(table) => table.get_hover_content(
                accessors,
                value_schema,
                toml_version,
                position,
                keys,
                definitions,
            ),
        }
    }
}

pub fn get_one_of_hover_content<T>(
    value: &T,
    accessors: &Vec<schema_store::Accessor>,
    one_of_schema: &schema_store::OneOfSchema,
    toml_version: config::TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    definitions: &schema_store::SchemaDefinitions,
) -> Option<HoverContent>
where
    T: GetHoverContent + Debug,
{
    let mut hover_contents = ahash::AHashSet::new();
    if let Ok(mut schemas) = one_of_schema.schemas.write() {
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            tracing::debug!("OneOf value: {:?}", value);
            tracing::debug!("OneOf schema: {:?}", value_schema);
            if let Some(hover_content) = value.get_hover_content(
                accessors,
                Some(&value_schema),
                toml_version,
                position,
                keys,
                definitions,
            ) {
                if hover_content.value_type != ValueType::Null {
                    hover_contents.insert(hover_content);
                }
            }
        }
    }
    if hover_contents.len() == 1 {
        hover_contents.into_iter().next()
    } else {
        None
    }
}

pub fn get_any_of_hover_content<T>(
    value: &T,
    accessors: &Vec<schema_store::Accessor>,
    any_of_schema: &schema_store::AnyOfSchema,
    toml_version: config::TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    definitions: &schema_store::SchemaDefinitions,
) -> Option<HoverContent>
where
    T: GetHoverContent,
{
    if let Ok(mut schemas) = any_of_schema.schemas.write() {
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            if let Some(hover_content) = value.get_hover_content(
                accessors,
                Some(&value_schema),
                toml_version,
                position,
                keys,
                definitions,
            ) {
                return Some(hover_content);
            }
        }
    }
    None
}
