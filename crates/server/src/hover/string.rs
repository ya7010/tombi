use schema_store::{Accessors, ValueSchema};

use super::GetHoverContent;

impl GetHoverContent for document_tree::String {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        value_schema: Option<&schema_store::ValueSchema>,
        toml_version: config::TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<super::HoverContent> {
        let value_type = schema_store::ValueType::String;

        tracing::debug!(
            "String::get_hover_content: value_schema: {:?}",
            value_schema
        );
        match value_schema {
            Some(ValueSchema::String(schema)) => {
                return Some(super::HoverContent {
                    title: schema.title.clone(),
                    description: schema.description.clone(),
                    keys: Accessors::new(accessors.clone()),
                    value_type,
                    ..Default::default()
                })
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
        };

        Some(super::HoverContent {
            keys: Accessors::new(accessors.clone()),
            value_type,
            ..Default::default()
        })
    }
}
