use super::{value::get_any_of_hover_content, GetHoverContent};

impl GetHoverContent for document_tree::Float {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        value_schema: Option<&schema_store::ValueSchema>,
        toml_version: config::TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<super::HoverContent> {
        let value_type = schema_store::ValueType::Float;

        match value_schema {
            Some(schema_store::ValueSchema::Float(schema)) => {
                return Some(super::HoverContent {
                    title: schema.title.clone(),
                    description: schema.description.clone(),
                    keys: schema_store::Accessors::new(accessors.clone()),
                    value_type,
                    ..Default::default()
                })
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
            _ => {}
        }

        Some(super::HoverContent {
            keys: schema_store::Accessors::new(accessors.clone()),
            value_type,
            ..Default::default()
        })
    }
}
