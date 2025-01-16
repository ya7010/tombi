use super::GetHoverContent;

impl GetHoverContent for document_tree::Array {
    fn get_hover_content(
        &self,
        accessors: &Vec<schema_store::Accessor>,
        value_schema: Option<&schema_store::ValueSchema>,
        toml_version: config::TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<super::HoverContent> {
        for (index, value) in self.values().iter().enumerate() {
            if value.range().contains(position) {
                let accessor = schema_store::Accessor::Index(index);

                return value.get_hover_content(
                    &accessors
                        .clone()
                        .into_iter()
                        .chain(std::iter::once(accessor))
                        .collect(),
                    value_schema,
                    toml_version,
                    position,
                    keys,
                    definitions,
                );
            }
        }

        let keys = schema_store::Accessors::new(accessors.clone());
        let value_type = schema_store::ValueType::Array;

        if let Some(schema_store::ValueSchema::Array(schema)) = value_schema {
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
