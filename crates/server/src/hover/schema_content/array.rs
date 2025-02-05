use schema_store::ArraySchema;

use super::SchemaHoverContent;

impl SchemaHoverContent for ArraySchema {
    fn schema_content(&self) -> Option<String> {
        let mut content = String::new();

        if let Some(min_items) = self.min_items {
            content.push_str(&format!("Minimum Items: `{}`\n\n", min_items));
        }

        if let Some(max_items) = self.max_items {
            content.push_str(&format!("Maximum Items: `{}`\n\n", max_items));
        }

        if let Some(unique_items) = self.unique_items {
            content.push_str(&format!("Unique Items: `{}`\n\n", unique_items));
        }

        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }
}
