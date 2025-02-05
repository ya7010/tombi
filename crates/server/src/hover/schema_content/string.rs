use schema_store::StringSchema;

use super::SchemaHoverContent;

impl SchemaHoverContent for StringSchema {
    fn schema_content(&self) -> Option<String> {
        let mut content = String::new();

        if let Some(min_length) = self.min_length {
            content.push_str(&format!("Minimum Length: `\"{}\"`\n\n", min_length));
        }

        if let Some(max_length) = self.max_length {
            content.push_str(&format!("Maximum Length: `\"{}\"`\n\n", max_length));
        }

        if let Some(pattern) = &self.pattern {
            content.push_str(&format!("Pattern: `\"{}\"`\n\n", pattern));
        }

        if let Some(enumerate) = &self.enumerate {
            content.push_str("Enumerated Values:\n\n");
            for value in enumerate {
                content.push_str(&format!("- `\"{}\"`\n\n", value));
            }
            content.push_str("\n");
        }

        if let Some(default) = &self.default {
            content.push_str(&format!("Default: `\"{}\"`\n\n", default));
        }

        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }
}
