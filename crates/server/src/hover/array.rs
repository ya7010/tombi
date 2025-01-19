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
        for (index, value) in self.values().iter().enumerate() {
            if value.range().contains(position) {
                let accessor = Accessor::Index(index);

                match value_schema {
                    Some(ValueSchema::Array(array)) => {
                        if let Some(items) = &array.items {
                            if let Ok(mut item_schema) = items.write() {
                                let Some(mut hover_content) = value.get_hover_content(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    item_schema.resolve(definitions).ok(),
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
                                    if let Some(title) = &array.title {
                                        hover_content.title = Some(title.clone());
                                    }
                                    if let Some(description) = &array.description {
                                        hover_content.description = Some(description.clone());
                                    }
                                }
                                return Some(hover_content);
                            }
                        }
                    }
                    Some(ValueSchema::OneOf(one_of_schema)) => {
                        if let Some(hover_content) = get_one_of_hover_content(
                            self,
                            accessors,
                            one_of_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                        ) {
                            return Some(hover_content);
                        }
                    }
                    Some(ValueSchema::AnyOf(any_of_schema)) => {
                        if let Some(hover_content) = get_any_of_hover_content(
                            self,
                            accessors,
                            any_of_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                        ) {
                            return Some(hover_content);
                        }
                    }
                    Some(ValueSchema::AllOf(all_of_schema)) => {
                        if let Some(hover_content) = get_all_of_hover_content(
                            self,
                            accessors,
                            all_of_schema,
                            toml_version,
                            position,
                            keys,
                            schema_url,
                            definitions,
                        ) {
                            return Some(hover_content);
                        }
                    }
                    Some(_) => return None,
                    None => {}
                }
                return value.get_hover_content(
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
                );
            }
        }

        match value_schema {
            Some(ValueSchema::Array(array)) => {
                return Some(HoverContent {
                    title: array.title.clone(),
                    description: array.description.clone(),
                    accessors: Accessors::new(accessors.clone()),
                    value_type: ValueType::Array,
                    enumerated_values: vec![],
                    schema_url: schema_url.cloned(),
                    range: Some(self.range()),
                });
            }
            Some(ValueSchema::OneOf(one_of_schema)) => {
                if let Some(hover_content) = get_one_of_hover_content(
                    self,
                    accessors,
                    one_of_schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                ) {
                    return Some(hover_content);
                }
            }
            Some(ValueSchema::AnyOf(any_of_schema)) => {
                if let Some(hover_content) = get_any_of_hover_content(
                    self,
                    accessors,
                    any_of_schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                ) {
                    return Some(hover_content);
                }
            }
            Some(ValueSchema::AllOf(all_of_schema)) => {
                if let Some(hover_content) = get_all_of_hover_content(
                    self,
                    accessors,
                    all_of_schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                ) {
                    return Some(hover_content);
                }
            }
            Some(_) => return None,
            None => {}
        }
        Some(super::HoverContent {
            title: None,
            description: None,
            accessors: Accessors::new(accessors.clone()),
            value_type: ValueType::Array,
            enumerated_values: vec![],
            schema_url: None,
            range: Some(self.range()),
        })
    }
}
