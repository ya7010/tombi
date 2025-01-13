use super::GetHoverContent;

impl GetHoverContent for document_tree::Float {
    fn get_hover_content(
        &self,
        accessors: &mut Vec<schema_store::Accessor>,
        value_schema: Option<&schema_store::ValueSchema>,
        _toml_version: config::TomlVersion,
        _position: text::Position,
        _keys: &[document_tree::Key],
        _definitions: Option<&schema_store::SchemaDefinitions>,
    ) -> Option<super::HoverContent> {
        let keys = schema_store::Accessors::new(accessors.clone());
        let value_type = schema_store::ValueType::Float;

        if let Some(schema_store::ValueSchema::Float(schema)) = value_schema {
            return Some(super::HoverContent {
                title: schema.title.clone(),
                description: schema.description.clone(),
                keys,
                value_type,
                ..Default::default()
            });
        } else {
            Some(super::HoverContent {
                keys,
                value_type,
                ..Default::default()
            })
        }
    }
}
