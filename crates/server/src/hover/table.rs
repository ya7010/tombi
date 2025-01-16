use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};

use super::{GetHoverContent, HoverContent};

impl GetHoverContent for document_tree::Table {
    fn get_hover_content(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        definitions: &SchemaDefinitions,
    ) -> Option<HoverContent> {
        if let Some(key) = keys.first() {
            if let Some(value) = self.get(key) {
                let accessor = Accessor::Key(key.to_raw_text(toml_version));

                match value_schema {
                    Some(ValueSchema::Table(table)) => {
                        if let Some(mut property) = table.properties.get_mut(&accessor) {
                            return value.get_hover_content(
                                &accessors
                                    .clone()
                                    .into_iter()
                                    .chain(std::iter::once(accessor))
                                    .collect(),
                                property.resolve(definitions).ok(),
                                toml_version,
                                position,
                                &keys[1..],
                                definitions,
                            );
                        }
                    }
                    Some(ValueSchema::AnyOf(any_of)) => {
                        if let Ok(mut schemas) = any_of.schemas.write() {
                            for referable_schema in schemas.iter_mut() {
                                let Ok(value_schema) = referable_schema.resolve(definitions) else {
                                    continue;
                                };
                                if let Some(hover_content) = self.get_hover_content(
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
                    }
                    _ => {}
                }

                return value.get_hover_content(
                    &accessors
                        .clone()
                        .into_iter()
                        .chain(std::iter::once(accessor))
                        .collect(),
                    value_schema,
                    toml_version,
                    position,
                    &keys[1..],
                    definitions,
                );
            } else {
                return None;
            }
        } else {
            match value_schema {
                Some(ValueSchema::Table(table)) => {
                    let mut hover_content = HoverContent {
                        keys: schema_store::Accessors::new(accessors.clone()),
                        value_type: schema_store::ValueType::Table,
                        ..Default::default()
                    };

                    if let Some(title) = &table.title {
                        hover_content.title = Some(title.clone());
                    }
                    if let Some(description) = &table.description {
                        hover_content.description = Some(description.clone());
                    }
                    return Some(hover_content);
                }
                Some(ValueSchema::AnyOf(any_of)) => {
                    if let Ok(mut schemas) = any_of.schemas.write() {
                        for referable_schema in schemas.iter_mut() {
                            let Ok(value_schema) = referable_schema.resolve(definitions) else {
                                continue;
                            };
                            if let Some(mut hover_content) = self.get_hover_content(
                                accessors,
                                Some(&value_schema),
                                toml_version,
                                position,
                                keys,
                                definitions,
                            ) {
                                if hover_content.title.is_none() {
                                    if let Some(title) = &any_of.title {
                                        hover_content.title = Some(title.clone());
                                    }
                                }
                                if hover_content.description.is_none() {
                                    if let Some(description) = &any_of.description {
                                        hover_content.description = Some(description.clone());
                                    }
                                }
                                return Some(hover_content);
                            }
                        }
                    }
                    None
                }
                Some(_) => return None,
                None => Some(HoverContent {
                    keys: schema_store::Accessors::new(accessors.clone()),
                    value_type: schema_store::ValueType::Table,
                    ..Default::default()
                }),
            }
        }
    }
}
