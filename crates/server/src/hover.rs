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
mod value;

use config::TomlVersion;
use dashmap::DashMap;
use schema_store::{Accessor, Accessors, DocumentSchema, ValueSchema, ValueType};
use std::{fmt::Debug, ops::Deref};

pub fn get_hover_content(
    root: &document_tree::Root,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    document_schema: Option<DocumentSchema>,
) -> Option<HoverContent> {
    let table = root.deref();
    let (value_schema, definitions) = match document_schema {
        Some(document_schema) => {
            let (value_schema, definitions) = document_schema.into();
            (Some(value_schema), definitions)
        }
        None => (None, DashMap::new()),
    };

    table.get_hover_content(
        &Vec::with_capacity(0),
        value_schema.as_ref(),
        toml_version,
        position,
        keys,
        &definitions,
    )
}

trait GetHoverContent {
    fn get_hover_content(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<HoverContent>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HoverContent {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keys: Accessors,
    pub value_type: ValueType,
    pub enumerated_values: Vec<String>,
    pub schema_url: Option<tower_lsp::lsp_types::Url>,
    pub range: Option<text::Range>,
}

impl HoverContent {
    pub fn into_nullable(mut self) -> HoverContent {
        self.value_type = self.value_type.into_nullable();
        self
    }
}

impl std::fmt::Display for HoverContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const SECTION_SEPARATOR: &str = "-----";

        if let Some(title) = &self.title {
            writeln!(f, "#### {}\n", title)?;
        }

        if let Some(description) = &self.description {
            writeln!(f, "{}\n", description)?;
        }

        if self.title.is_some() || self.description.is_some() {
            writeln!(f, "{}\n", SECTION_SEPARATOR)?;
        }

        writeln!(f, "Keys: `{}`\n", self.keys)?;
        writeln!(f, "Value: `{}`\n", self.value_type)?;

        if !self.enumerated_values.is_empty() {
            writeln!(f, "Allowed Values:\n")?;
            for value in &self.enumerated_values {
                writeln!(f, "- `{}`", value)?;
            }
            writeln!(f)?;
        }

        if let Some(schema_url) = &self.schema_url {
            if let Some(schema_filename) = get_schema_name(schema_url) {
                writeln!(f, "Source: [{schema_filename}]({schema_url})\n",)?;
            }
        }

        Ok(())
    }
}

impl From<HoverContent> for tower_lsp::lsp_types::Hover {
    fn from(value: HoverContent) -> Self {
        tower_lsp::lsp_types::Hover {
            contents: tower_lsp::lsp_types::HoverContents::Markup(
                tower_lsp::lsp_types::MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: value.to_string(),
                },
            ),
            range: value.range.map(Into::into),
        }
    }
}

fn get_schema_name(schema_url: &tower_lsp::lsp_types::Url) -> Option<&str> {
    if let Some(path) = schema_url.path().split('/').last() {
        if !path.is_empty() {
            return Some(path);
        }
    }
    schema_url.host_str()
}

fn get_one_of_hover_content<T>(
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
            if let Some(hover_content) = value.get_hover_content(
                accessors,
                Some(&value_schema),
                toml_version,
                position,
                keys,
                definitions,
            ) {
                if keys.is_empty() {
                    value_types.insert(value_schema.value_type());
                    if hover_content.value_type != ValueType::Null {
                        hover_contents.insert(hover_content);
                    }
                } else {
                    return Some(hover_content);
                }
            }
        }
    }

    if hover_contents.len() == 1 {
        hover_contents.into_iter().next().map(|mut hover_content| {
            if hover_content.title.is_none() && hover_content.description.is_none() {
                if let Some(title) = &one_of_schema.title {
                    hover_content.title = Some(title.clone());
                }
                if let Some(description) = &one_of_schema.description {
                    hover_content.description = Some(description.clone());
                }
            }

            if value_types.len() == 1 {
                hover_content.value_type = value_types.into_iter().next().unwrap();
            } else {
                hover_content.value_type = ValueType::OneOf(value_types.into_iter().collect());
            }

            hover_content
        })
    } else {
        None
    }
}

fn get_any_of_hover_content<T>(
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

            if keys.is_empty() {
                value_types.insert(value_schema.value_type());
            }

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
                if content.title.is_none() && content.description.is_none() {
                    if let Some(title) = &any_of_schema.title {
                        content.title = Some(title.clone());
                    }
                    if let Some(description) = &any_of_schema.description {
                        content.description = Some(description.clone());
                    }
                }

                hover_content = Some(content);
            }
        }
        if let Some(mut hover_content) = hover_content {
            if value_types.len() == 1 {
                hover_content.value_type = value_types.into_iter().next().unwrap();
            } else {
                hover_content.value_type = ValueType::AnyOf(value_types.into_iter().collect());
            }
            return Some(hover_content);
        }
    }
    None
}

fn get_all_of_hover_content<T>(
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

    if title.is_none() && description.is_none() {
        if let Some(t) = &all_of_schema.title {
            title = Some(t.clone());
        }
        if let Some(d) = &all_of_schema.description {
            description = Some(d.clone());
        }
    }

    let value_type = if value_types.len() == 1 {
        value_types.into_iter().next().unwrap()
    } else {
        ValueType::AllOf(value_types.into_iter().collect())
    };

    Some(HoverContent {
        title,
        description,
        keys: schema_store::Accessors::new(accessors.clone()),
        value_type,
        enumerated_values: Vec::new(),
        schema_url: None,
        range: None,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use tower_lsp::lsp_types::Url;

    #[rstest]
    #[case("https://json.schemastore.org/tombi.schema.json")]
    #[case("file://./folder/tombi.schema.json")]
    #[case("file://./tombi.schema.json")]
    #[case("file://tombi.schema.json")]
    fn url_content(#[case] url: &str) {
        let url = Url::parse(url).unwrap();
        assert_eq!(get_schema_name(&url).unwrap(), "tombi.schema.json");
    }

    #[test]
    fn any_of_array_null() {
        let value_type = ValueType::AnyOf(
            vec![ValueType::Array, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Array?");
    }

    #[test]
    fn one_of_array_null() {
        let value_type = ValueType::OneOf(
            vec![ValueType::Array, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Array?");
    }

    #[test]
    fn all_of_array_null() {
        let value_type = ValueType::AllOf(
            vec![ValueType::Array, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Array?");
    }

    #[test]
    fn nullable_one_of() {
        let value_type = ValueType::OneOf(
            vec![ValueType::Array, ValueType::Table, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "(Array ^ Table)?");
    }

    #[test]
    fn nullable_any_of() {
        let value_type = ValueType::AnyOf(
            vec![ValueType::Array, ValueType::Table, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "(Array | Table)?");
    }

    #[test]
    fn nullable_all_of() {
        let value_type = ValueType::AllOf(
            vec![ValueType::Array, ValueType::Table, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "(Array & Table)?");
    }
}
