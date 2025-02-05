use schema_store::IntegerSchema;

use super::SchemaHoverContent;

impl SchemaHoverContent for IntegerSchema {
    fn schema_content(&self) -> Option<String> {
        let mut content = String::new();

        if let Some(minimum) = self.minimum {
            content.push_str(&format!("Minimum: `{}`\n\n", minimum));
        }

        if let Some(exclusive_minimum) = self.exclusive_minimum {
            content.push_str(&format!("Exclusive Minimum: `{}`\n\n", exclusive_minimum));
        }

        if let Some(maximum) = self.maximum {
            content.push_str(&format!("Maximum: `{}`\n\n", maximum));
        }

        if let Some(exclusive_maximum) = self.exclusive_maximum {
            content.push_str(&format!("Exclusive Maximum: `{}`\n\n", exclusive_maximum));
        }

        if let Some(multiple_of) = self.multiple_of {
            content.push_str(&format!("Multiple of: `{}`\n\n", multiple_of));
        }

        if let Some(enumerate) = &self.enumerate {
            content.push_str("Enumerated Values:\n\n");
            for value in enumerate {
                content.push_str(&format!("- `{}`\n\n", value));
            }
            content.push_str("\n");
        }

        if let Some(default) = self.default {
            content.push_str(&format!("Default: `{}`\n\n", default));
        }

        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }
}
