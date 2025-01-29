use config::TomlVersion;
use schema_store::{Accessor, Accessors, SchemaDefinitions, TableSchema, ValueSchema, ValueType};
use tower_lsp::lsp_types::Url;

use super::{
    all_of::get_all_of_hover_content, any_of::get_any_of_hover_content,
    one_of::get_one_of_hover_content, GetHoverContent, HoverContent,
};

impl GetHoverContent for document_tree::Table {
    fn get_hover_content(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
    ) -> Option<HoverContent> {
        match value_schema {
            Some(ValueSchema::Table(table_schema)) => {
                if let Some(key) = keys.first() {
                    if let Some(value) = self.get(key) {
                        let key_str = key.to_raw_text(toml_version);
                        let accessor = Accessor::Key(key_str.clone());

                        if let Some(mut property) = table_schema.properties.get_mut(&accessor) {
                            let required = table_schema
                                .required
                                .as_ref()
                                .map(|r| r.contains(&key_str))
                                .unwrap_or(false);

                            return value
                                .get_hover_content(
                                    &accessors
                                        .clone()
                                        .into_iter()
                                        .chain(std::iter::once(accessor))
                                        .collect(),
                                    property.resolve(definitions).ok(),
                                    toml_version,
                                    position,
                                    &keys[1..],
                                    schema_url,
                                    definitions,
                                )
                                .map(|hover_content| {
                                    if keys.len() == 1
                                        && !required
                                        && hover_content
                                            .accessors
                                            .last()
                                            .map(|accessor| accessor.is_key())
                                            .unwrap_or_default()
                                    {
                                        hover_content.into_nullable()
                                    } else {
                                        hover_content
                                    }
                                });
                        }
                        if let Some(pattern_properties) = &table_schema.pattern_properties {
                            for mut pattern_property in pattern_properties.iter_mut() {
                                let property_key = pattern_property.key();
                                if let Ok(pattern) = regex::Regex::new(property_key) {
                                    if pattern.is_match(&key_str) {
                                        let property_schema = pattern_property.value_mut();

                                        return value
                                            .get_hover_content(
                                                &accessors
                                                    .clone()
                                                    .into_iter()
                                                    .chain(std::iter::once(accessor))
                                                    .collect(),
                                                property_schema.resolve(definitions).ok(),
                                                toml_version,
                                                position,
                                                &keys[1..],
                                                schema_url,
                                                definitions,
                                            )
                                            .map(|hover_content| {
                                                if keys.len() == 1
                                                    && hover_content
                                                        .accessors
                                                        .last()
                                                        .map(|accessor| accessor.is_key())
                                                        .unwrap_or_default()
                                                {
                                                    hover_content.into_nullable()
                                                } else {
                                                    hover_content
                                                }
                                            });
                                    }
                                };
                            }
                        }
                        if let Some(additiona_property_schema) =
                            &table_schema.additional_property_schema
                        {
                            if let Ok(mut additiona_property_schema) =
                                additiona_property_schema.write()
                            {
                                return value
                                    .get_hover_content(
                                        &accessors
                                            .clone()
                                            .into_iter()
                                            .chain(std::iter::once(accessor))
                                            .collect(),
                                        additiona_property_schema.resolve(definitions).ok(),
                                        toml_version,
                                        position,
                                        &keys[1..],
                                        schema_url,
                                        definitions,
                                    )
                                    .map(|hover_content| {
                                        if keys.len() == 1
                                            && hover_content
                                                .accessors
                                                .last()
                                                .map(|accessor| accessor.is_key())
                                                .unwrap_or_default()
                                        {
                                            hover_content.into_nullable()
                                        } else {
                                            hover_content
                                        }
                                    });
                            }
                        }

                        value.get_hover_content(
                            &accessors
                                .clone()
                                .into_iter()
                                .chain(std::iter::once(accessor))
                                .collect(),
                            None,
                            toml_version,
                            position,
                            &keys[1..],
                            schema_url,
                            definitions,
                        )
                    } else {
                        None
                    }
                } else {
                    table_schema
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
                            hover_content.range = Some(self.range());
                            hover_content
                        })
                }
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
            None => {
                if let Some(key) = keys.first() {
                    if let Some(value) = self.get(key) {
                        let accessor = Accessor::Key(key.to_raw_text(toml_version));

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
                            schema_url,
                            definitions,
                        );
                    }
                }
                Some(HoverContent {
                    title: None,
                    description: None,
                    accessors: Accessors::new(accessors.clone()),
                    value_type: ValueType::Table,
                    enumerated_values: vec![],
                    schema_url: None,
                    range: Some(self.range()),
                })
            }
        }
    }
}

impl GetHoverContent for TableSchema {
    fn get_hover_content(
        &self,
        accessors: &Vec<Accessor>,
        _value_schema: Option<&ValueSchema>,
        _toml_version: TomlVersion,
        _position: text::Position,
        _keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        _definitions: &schema_store::SchemaDefinitions,
    ) -> Option<HoverContent> {
        Some(HoverContent {
            title: self.title.clone(),
            description: self.description.clone(),
            accessors: Accessors::new(accessors.clone()),
            value_type: ValueType::Table,
            enumerated_values: vec![],
            schema_url: schema_url.cloned(),
            range: None,
        })
    }
}
