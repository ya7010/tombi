use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};

use super::{GetHoverContent, HoverContent};

impl GetHoverContent for document_tree::Table {
    fn get_hover_content(
        &self,
        accessors: &mut Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        definitions: Option<&SchemaDefinitions>,
    ) -> Option<HoverContent> {
        if let Some(key) = keys.first() {
            accessors.push(Accessor::Key(key.to_raw_text(toml_version)));
            if let Some(value) = self.get(key) {
                return value.get_hover_content(
                    accessors,
                    value_schema,
                    toml_version,
                    position,
                    &keys[1..],
                    definitions,
                );
            }
        }

        let keys = schema_store::Accessors::new(accessors.clone());
        let value_type = schema_store::ValueType::Table;

        if let Some(schema_store::ValueSchema::Table(schema)) = value_schema {
            Some(super::HoverContent {
                title: schema.title.clone(),
                description: schema.description.clone(),
                keys,
                value_type,
                ..Default::default()
            })
        } else {
            Some(super::HoverContent {
                keys,
                value_type,
                ..Default::default()
            })
        }
    }
}
