use schema_store::TableSchema;

use super::SchemaHoverContent;

impl SchemaHoverContent for TableSchema {
    fn schema_content(&self) -> Option<String> {
        let mut content = String::new();

        if let Some(min_properties) = self.min_properties {
            content.push_str(&format!("Minimum Keys: `{}`\n\n", min_properties));
        }

        if let Some(max_properties) = self.max_properties {
            content.push_str(&format!("Maximum Keys: `{}`\n\n", max_properties));
        }

        if let Some(pattern_properties) = &self.pattern_properties {
            content.push_str("Key Patterns:\n\n");
            for pattern_property in pattern_properties.iter() {
                content.push_str(&format!("- `{}`\n\n", pattern_property.key()));
            }
        }

        if self.additional_properties {
            content.push_str(&format!(
                "Additional Keys: `{}`\n\n",
                self.additional_properties
            ));
        }

        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }
}
