use schema_store::LocalTimeSchema;

use super::SchemaHoverContent;

impl SchemaHoverContent for LocalTimeSchema {
    fn schema_content(&self) -> Option<String> {
        let mut content = String::new();

        if let Some(enumerate) = &self.enumerate {
            content.push_str("Enumerated Values:\n\n");
            for value in enumerate {
                content.push_str(&format!("- `{}`\n\n", value));
            }
            content.push_str("\n");
        }

        if let Some(default) = &self.default {
            content.push_str(&format!("Default: `{}`\n\n", default));
        }

        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }
}
