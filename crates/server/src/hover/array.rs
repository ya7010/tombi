use super::{
    value::{get_all_of_hover_content, get_any_of_hover_content, get_one_of_hover_content},
    GetHoverContent, HoverContent,
};

impl GetHoverContent for document_tree::Array {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        value_schema: Option<&schema_store::ValueSchema>,
        toml_version: config::TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<super::HoverContent> {
        for (index, value) in self.values().iter().enumerate() {
            if value.range().contains(position) {
                let accessor = schema_store::Accessor::Index(index);

                match value_schema {
                    Some(schema_store::ValueSchema::Array(array)) => {
                        if let Some(items) = &array.items {
                            if let Ok(mut item) = items.write() {
                                let Some(mut hover_content) = value.get_hover_content(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    item.resolve(definitions).ok(),
                                    toml_version,
                                    position,
                                    keys,
                                    definitions,
                                ) else {
                                    return None;
                                };

                                if hover_content.title.is_none() {
                                    if let Some(title) = &array.title {
                                        hover_content.title = Some(title.clone());
                                    }
                                }
                                if hover_content.description.is_none() {
                                    if let Some(description) = &array.description {
                                        hover_content.description = Some(description.clone());
                                    }
                                }
                                return Some(hover_content);
                            }
                        }
                    }
                    Some(schema_store::ValueSchema::OneOf(one_of_schema)) => {
                        if let Some(hover_content) = get_one_of_hover_content(
                            self,
                            accessors,
                            one_of_schema,
                            toml_version,
                            position,
                            keys,
                            definitions,
                        ) {
                            return Some(hover_content);
                        }
                    }
                    Some(schema_store::ValueSchema::AnyOf(any_of_schema)) => {
                        if let Some(hover_content) = get_any_of_hover_content(
                            self,
                            accessors,
                            any_of_schema,
                            toml_version,
                            position,
                            keys,
                            definitions,
                        ) {
                            return Some(hover_content);
                        }
                    }
                    Some(schema_store::ValueSchema::AllOf(all_of_schema)) => {
                        if let Some(hover_content) = get_all_of_hover_content(
                            self,
                            accessors,
                            all_of_schema,
                            toml_version,
                            position,
                            keys,
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
                    definitions,
                );
            }
        }

        match value_schema {
            Some(schema_store::ValueSchema::Array(array)) => {
                let mut hover_content = HoverContent {
                    keys: schema_store::Accessors::new(accessors.clone()),
                    value_type: schema_store::ValueType::Array,
                    ..Default::default()
                };

                if let Some(title) = &array.title {
                    hover_content.title = Some(title.clone());
                }
                if let Some(description) = &array.description {
                    hover_content.description = Some(description.clone());
                }
                return Some(hover_content);
            }
            Some(schema_store::ValueSchema::OneOf(one_of_schema)) => {
                if let Some(hover_content) = get_one_of_hover_content(
                    self,
                    accessors,
                    one_of_schema,
                    toml_version,
                    position,
                    keys,
                    definitions,
                ) {
                    return Some(hover_content);
                }
            }
            Some(schema_store::ValueSchema::AnyOf(any_of_schema)) => {
                if let Some(hover_content) = get_any_of_hover_content(
                    self,
                    accessors,
                    any_of_schema,
                    toml_version,
                    position,
                    keys,
                    definitions,
                ) {
                    return Some(hover_content);
                }
            }
            Some(schema_store::ValueSchema::AllOf(all_of_schema)) => {
                if let Some(hover_content) = get_all_of_hover_content(
                    self,
                    accessors,
                    all_of_schema,
                    toml_version,
                    position,
                    keys,
                    definitions,
                ) {
                    return Some(hover_content);
                }
            }
            Some(_) => return None,
            None => {}
        }
        Some(super::HoverContent {
            keys: schema_store::Accessors::new(accessors.clone()),
            value_type: schema_store::ValueType::Array,
            ..Default::default()
        })
    }
}
