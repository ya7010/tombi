use schema_store::ValueSchema;
use tower_lsp::lsp_types::Url;

use super::{
    get_all_of_hover_content, get_any_of_hover_content, get_one_of_hover_content, GetHoverContent,
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
            Some(ValueSchema::OffsetDateTime(schema)) => Some(super::HoverContent {
                title: schema.title.clone(),
                description: schema.description.clone(),
                accessors: schema_store::Accessors::new(accessors.clone()),
                value_type: schema_store::ValueType::OffsetDateTime,
                enumerated_values: schema
                    .enumerate
                    .as_ref()
                    .map(|v| v.iter().map(ToString::to_string).collect())
                    .unwrap_or_default(),
                schema_url: schema_url.cloned(),
                range: Some(self.range()),
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
