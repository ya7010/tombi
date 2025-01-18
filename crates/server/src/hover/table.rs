use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};

use super::{
    value::{get_all_of_hover_content, get_any_of_hover_content, get_one_of_hover_content},
    GetHoverContent, HoverContent,
};

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
                    Some(ValueSchema::Table(table_schema)) => {
                        if let Some(mut property) = table_schema.properties.get_mut(&accessor) {
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
                        } else if let Some(additiona_property_schema) =
                            &table_schema.additional_property_schema
                        {
                            if let Ok(mut additiona_property_schema) =
                                additiona_property_schema.write()
                            {
                                return value.get_hover_content(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    additiona_property_schema.resolve(definitions).ok(),
                                    toml_version,
                                    position,
                                    &keys[1..],
                                    definitions,
                                );
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
                    &keys[1..],
                    definitions,
                );
            } else {
                return None;
            }
        } else {
            match value_schema {
                Some(ValueSchema::Table(table)) => {
                    return Some(HoverContent {
                        title: table.title.clone(),
                        description: table.description.clone(),
                        keys: schema_store::Accessors::new(accessors.clone()),
                        value_type: schema_store::ValueType::Table,
                        enumerated_values: vec![],
                        schema_url: None,
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
                        definitions,
                    ) {
                        return Some(hover_content);
                    }
                }
                Some(_) => return None,
                None => {}
            }
        }

        Some(HoverContent {
            title: None,
            description: None,
            keys: schema_store::Accessors::new(accessors.clone()),
            value_type: schema_store::ValueType::Table,
            enumerated_values: vec![],
            schema_url: None,
            range: Some(self.range()),
        })
    }
}
