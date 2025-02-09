use schema_store::{IntegerSchema, SchemaUrl, ValueSchema};

use crate::hover::{
    all_of::get_all_of_hover_content, any_of::get_any_of_hover_content,
    constraints::DataConstraints, default_value::DefaultValue, one_of::get_one_of_hover_content,
    GetHoverContent, HoverContent,
};

impl GetHoverContent for document_tree::Integer {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: config::TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&SchemaUrl>,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<HoverContent> {
        match value_schema {
            Some(ValueSchema::Integer(integer_schema)) => integer_schema
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
                }),
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
            None => Some(HoverContent {
                title: None,
                description: None,
                accessors: schema_store::Accessors::new(accessors.clone()),
                value_type: schema_store::ValueType::Integer,
                constraints: None,
                schema_url: None,
                range: Some(self.range()),
            }),
        }
    }
}

impl GetHoverContent for IntegerSchema {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        _value_schema: Option<&ValueSchema>,
        _toml_version: config::TomlVersion,
        _position: text::Position,
        _keys: &[document_tree::Key],
        schema_url: Option<&SchemaUrl>,
        _definitions: &schema_store::SchemaDefinitions,
    ) -> Option<HoverContent> {
        Some(HoverContent {
            title: self.title.clone(),
            description: self.description.clone(),
            accessors: schema_store::Accessors::new(accessors.clone()),
            value_type: schema_store::ValueType::Integer,
            constraints: Some(DataConstraints {
                default: self.default.map(DefaultValue::Integer),
                enumerate: self.enumerate.as_ref().map(|value| {
                    value
                        .iter()
                        .map(|value| DefaultValue::Integer(*value))
                        .collect()
                }),
                minimum: self.minimum.map(DefaultValue::Integer),
                maximum: self.maximum.map(DefaultValue::Integer),
                exclusive_minimum: self.exclusive_minimum.map(DefaultValue::Integer),
                exclusive_maximum: self.exclusive_maximum.map(DefaultValue::Integer),
                multiple_of: self.multiple_of.map(DefaultValue::Integer),
                ..Default::default()
            }),
            schema_url: schema_url.cloned(),
            range: None,
        })
    }
}
