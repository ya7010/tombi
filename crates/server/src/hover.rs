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
use tower_lsp::lsp_types::Url;

pub fn get_hover_content(
    tree: &document_tree::DocumentTree,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    document_schema: Option<&DocumentSchema>,
) -> Option<HoverContent> {
    let table = tree.deref();
    match document_schema {
        Some(document_schema) => table.get_hover_content(
            &Vec::with_capacity(0),
            Some(document_schema.value_schema()),
            toml_version,
            position,
            keys,
            Some(&document_schema.schema_url),
            &document_schema.definitions,
        ),
        None => table.get_hover_content(
            &Vec::with_capacity(0),
            None,
            toml_version,
            position,
            keys,
            None,
            &DashMap::new(),
        ),
    }
}

trait GetHoverContent {
    fn get_hover_content(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<HoverContent>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HoverContent {
    pub title: Option<String>,
    pub description: Option<String>,
    pub accessors: Accessors,
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

        writeln!(f, "Keys: `{}`\n", self.accessors)?;
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
                writeln!(f, "Schema: [{schema_filename}]({schema_url})\n",)?;
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
    schema_url: Option<&Url>,
    definitions: &schema_store::SchemaDefinitions,
) -> Option<HoverContent>
where
    T: GetHoverContent + document_tree::ValueImpl,
{
    let mut one_hover_contents = ahash::AHashSet::new();
    if let Ok(mut schemas) = one_of_schema.schemas.write() {
        let mut value_type_set = indexmap::IndexSet::new();
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            value_type_set.insert(value_schema.value_type());
        }

        let value_type = if value_type_set.len() == 1 {
            value_type_set.into_iter().next().unwrap()
        } else {
            ValueType::OneOf(value_type_set.into_iter().collect())
        };

        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            if let Some(mut hover_content) = value.get_hover_content(
                accessors,
                Some(&value_schema),
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ) {
                if hover_content.title.is_none() && hover_content.description.is_none() {
                    if let Some(title) = &one_of_schema.title {
                        hover_content.title = Some(title.clone());
                    }
                    if let Some(description) = &one_of_schema.description {
                        hover_content.description = Some(description.clone());
                    }
                }

                if keys.is_empty() && accessors == hover_content.accessors.as_ref() {
                    hover_content.value_type = value_type.clone();
                }

                if value_schema.value_type() == schema_store::ValueType::Array
                    && hover_content.value_type != schema_store::ValueType::Array
                {
                    return Some(hover_content);
                }

                one_hover_contents.insert(hover_content);
            }
        }
    }

    if one_hover_contents.len() == 1 {
        one_hover_contents
            .into_iter()
            .next()
            .map(|mut hover_content| {
                if hover_content.title.is_none() && hover_content.description.is_none() {
                    if let Some(title) = &one_of_schema.title {
                        hover_content.title = Some(title.clone());
                    }
                    if let Some(description) = &one_of_schema.description {
                        hover_content.description = Some(description.clone());
                    }
                }

                hover_content
            })
    } else {
        Some(HoverContent {
            title: None,
            description: None,
            accessors: schema_store::Accessors::new(accessors.clone()),
            value_type: value.value_type().into(),
            enumerated_values: Vec::new(),
            schema_url: None,
            range: None,
        })
    }
}

fn get_any_of_hover_content<T>(
    value: &T,
    accessors: &Vec<schema_store::Accessor>,
    any_of_schema: &schema_store::AnyOfSchema,
    toml_version: config::TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    schema_url: Option<&Url>,
    definitions: &schema_store::SchemaDefinitions,
) -> Option<HoverContent>
where
    T: GetHoverContent + document_tree::ValueImpl,
{
    if let Ok(mut schemas) = any_of_schema.schemas.write() {
        let mut value_type_set = indexmap::IndexSet::new();
        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };
            value_type_set.insert(value_schema.value_type());
        }

        let value_type = if value_type_set.len() == 1 {
            value_type_set.into_iter().next().unwrap()
        } else {
            ValueType::AnyOf(value_type_set.into_iter().collect())
        };

        for referable_schema in schemas.iter_mut() {
            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                continue;
            };

            if let Some(mut hover_content) = value.get_hover_content(
                accessors,
                Some(&value_schema),
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ) {
                if hover_content.title.is_none() && hover_content.description.is_none() {
                    if let Some(title) = &any_of_schema.title {
                        hover_content.title = Some(title.clone());
                    }
                    if let Some(description) = &any_of_schema.description {
                        hover_content.description = Some(description.clone());
                    }
                }

                if keys.is_empty() && accessors == hover_content.accessors.as_ref() {
                    hover_content.value_type = value_type;
                }

                return Some(hover_content);
            }
        }
    };

    Some(HoverContent {
        title: None,
        description: None,
        accessors: schema_store::Accessors::new(accessors.clone()),
        value_type: value.value_type().into(),
        enumerated_values: Vec::new(),
        schema_url: None,
        range: None,
    })
}

fn get_all_of_hover_content<T>(
    value: &T,
    accessors: &Vec<schema_store::Accessor>,
    all_of_schema: &schema_store::AllOfSchema,
    toml_version: config::TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    schema_url: Option<&Url>,
    definitions: &schema_store::SchemaDefinitions,
) -> Option<HoverContent>
where
    T: GetHoverContent,
{
    let mut title_description_set = ahash::AHashSet::new();
    let mut value_type_set = indexmap::IndexSet::new();
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
                schema_url,
                definitions,
            ) {
                if hover_content.title.is_some() || hover_content.description.is_some() {
                    title_description_set.insert((
                        hover_content.title.clone(),
                        hover_content.description.clone(),
                    ));
                    value_type_set.insert(hover_content.value_type);
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

    let value_type = if value_type_set.len() == 1 {
        value_type_set.into_iter().next().unwrap()
    } else {
        ValueType::AllOf(value_type_set.into_iter().collect())
    };

    Some(HoverContent {
        title,
        description,
        accessors: schema_store::Accessors::new(accessors.clone()),
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
}
