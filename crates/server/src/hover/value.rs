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
    let mut value_types = indexmap::IndexSet::new();
    let mut hover_contents = ahash::AHashSet::new();
    if let Ok(mut schemas) = one_of_schema.schemas.write() {
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            value_types.insert(value_schema.value_type());

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
        hover_contents.into_iter().next().map(|mut hover_content| {
            if hover_content.title.is_none() {
                if let Some(title) = &one_of_schema.title {
                    hover_content.title = Some(title.clone());
                }
            }
            if hover_content.description.is_none() {
                if let Some(description) = &one_of_schema.description {
                    hover_content.description = Some(description.clone());
                }
            }
            hover_content.value_type = ValueType::OneOf(value_types.into_iter().collect());

            hover_content
        })
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
        let mut value_types = indexmap::IndexSet::new();
        let mut hover_content = None;
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            value_types.insert(value_schema.value_type());

            if hover_content.is_some() {
                continue;
            }

            if let Some(mut content) = value.get_hover_content(
                accessors,
                Some(&value_schema),
                toml_version,
                position,
                keys,
                definitions,
            ) {
                if content.title.is_none() {
                    if let Some(title) = &any_of_schema.title {
                        content.title = Some(title.clone());
                    }
                }
                if content.description.is_none() {
                    if let Some(description) = &any_of_schema.description {
                        content.description = Some(description.clone());
                    }
                }
                hover_content = Some(content);
            }
        }
        if let Some(mut hover_content) = hover_content {
            hover_content.value_type = ValueType::AnyOf(value_types.into_iter().collect());
            return Some(hover_content);
        }
    }
    None
}

pub fn get_all_of_hover_content<T>(
    value: &T,
    accessors: &Vec<schema_store::Accessor>,
    all_of_schema: &schema_store::AllOfSchema,
    toml_version: config::TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    definitions: &schema_store::SchemaDefinitions,
) -> Option<HoverContent>
where
    T: GetHoverContent,
{
    let mut title_description_set = ahash::AHashSet::new();
    let mut value_types = indexmap::IndexSet::new();
    if let Ok(mut schemas) = all_of_schema.schemas.write() {
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                return None;
            };
            if let Some(hover_content) = value.get_hover_content(
                accessors,
                Some(&value_schema),
                toml_version,
                position,
                keys,
                definitions,
            ) {
                if hover_content.title.is_some() || hover_content.description.is_some() {
                    title_description_set.insert((
                        hover_content.title.clone(),
                        hover_content.description.clone(),
                    ));
                    value_types.insert(hover_content.value_type);
                } else {
                    return None;
                }
            }
        }
    }

    let (mut title, mut description) = if title_description_set.len() == 1 {
        title_description_set.into_iter().next().unwrap()
    } else {
        (None, None)
    };

    if title.is_none() {
        if let Some(t) = &all_of_schema.title {
            title = Some(t.clone());
        }
    }

    if description.is_none() {
        if let Some(d) = &all_of_schema.description {
            description = Some(d.clone());
        }
    }

    Some(HoverContent {
        title,
        description,
        keys: schema_store::Accessors::new(accessors.clone()),
        value_type: schema_store::ValueType::AllOf(value_types.into_iter().collect()),
        ..Default::default()
    })
}
