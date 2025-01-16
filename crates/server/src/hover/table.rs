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
        definitions: &SchemaDefinitions,
    ) -> Option<HoverContent> {
        if let Some(key) = keys.first() {
            if let Some(value) = self.get(key) {
                let accessor = Accessor::Key(key.to_raw_text(toml_version));

                accessors.push(accessor);

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

        Some(HoverContent {
            keys,
            value_type,
            ..Default::default()
        })
    }
}
