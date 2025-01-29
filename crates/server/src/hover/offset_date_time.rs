use schema_store::{OffsetDateTimeSchema, ValueSchema};
use tower_lsp::lsp_types::Url;

use super::{
    all_of::get_all_of_hover_content, any_of::get_any_of_hover_content,
    one_of::get_one_of_hover_content, GetHoverContent,
};

impl GetHoverContent for document_tree::OffsetDateTime {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: config::TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<super::HoverContent> {
        match value_schema {
            Some(ValueSchema::OffsetDateTime(offset_date_time_schema)) => offset_date_time_schema
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
            None => Some(super::HoverContent {
                title: None,
                description: None,
                accessors: schema_store::Accessors::new(accessors.clone()),
                value_type: schema_store::ValueType::OffsetDateTime,
                enumerated_values: vec![],
                schema_url: None,
                range: Some(self.range()),
            }),
        }
    }
}

impl GetHoverContent for OffsetDateTimeSchema {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        _value_schema: Option<&ValueSchema>,
        _toml_version: config::TomlVersion,
        _position: text::Position,
        _keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        _definitions: &schema_store::SchemaDefinitions,
    ) -> Option<super::HoverContent> {
        Some(super::HoverContent {
            title: self.title.clone(),
            description: self.description.clone(),
            accessors: schema_store::Accessors::new(accessors.clone()),
            value_type: schema_store::ValueType::OffsetDateTime,
            enumerated_values: self
                .enumerate
                .as_ref()
                .map(|v| v.iter().map(ToString::to_string).collect())
                .unwrap_or_default(),
            schema_url: schema_url.cloned(),
            range: None,
        })
    }
}
