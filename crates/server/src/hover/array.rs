use schema_store::{Accessor, Accessors, ValueSchema, ValueType};
use tower_lsp::lsp_types::Url;

use super::{
    get_all_of_hover_content, get_any_of_hover_content, get_one_of_hover_content, GetHoverContent,
    HoverContent,
};

impl GetHoverContent for document_tree::Array {
    fn get_hover_content(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: config::TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<super::HoverContent> {
        match value_schema {
            Some(ValueSchema::Array(array_schema)) => {
                for (index, value) in self.values().iter().enumerate() {
                    if value.range().contains(position) {
                        let accessor = Accessor::Index(index);

                        return array_schema
                            .operate_item(
                                |item_schema| {
                                    let Some(mut hover_content) = value.get_hover_content(
                                        &accessors
                                            .clone()
                                            .into_iter()
                                            .chain(std::iter::once(accessor.clone()))
                                            .collect(),
                                        Some(item_schema),
                                        toml_version,
                                        position,
                                        keys,
                                        schema_url,
                                        definitions,
                                    ) else {
                                        return None;
                                    };

                                    if hover_content.title.is_none()
                                        && hover_content.description.is_none()
                                    {
                                        if let Some(title) = &array_schema.title {
                                            hover_content.title = Some(title.clone());
                                        }
                                        if let Some(description) = &array_schema.description {
                                            hover_content.description = Some(description.clone());
                                        }
                                    }
                                    Some(hover_content)
                                },
                                definitions,
                            )
                            .unwrap_or_else(|| {
                                value.get_hover_content(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    None,
                                    toml_version,
                                    position,
                                    keys,
                                    schema_url,
                                    definitions,
                                )
                            });
                    }
                }
                Some(HoverContent {
                    title: array_schema.title.clone(),
                    description: array_schema.description.clone(),
                    accessors: Accessors::new(accessors.clone()),
                    value_type: ValueType::Array,
                    enumerated_values: vec![],
                    schema_url: schema_url.cloned(),
                    range: Some(self.range()),
                })
            }
            Some(ValueSchema::OneOf(one_of_schema)) => get_one_of_hover_content(
                self,
                accessors,
                one_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Some(ValueSchema::AnyOf(any_of_schema)) => get_any_of_hover_content(
                self,
                accessors,
                any_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Some(ValueSchema::AllOf(all_of_schema)) => get_all_of_hover_content(
                self,
                accessors,
                all_of_schema,
                toml_version,
                position,
                keys,
                schema_url,
                definitions,
            ),
            Some(_) => None,
            None => Some(super::HoverContent {
                title: None,
                description: None,
                accessors: Accessors::new(accessors.clone()),
                value_type: ValueType::Array,
                enumerated_values: vec![],
                schema_url: None,
                range: Some(self.range()),
            }),
        }
    }
}
