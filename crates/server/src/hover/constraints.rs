pub struct CompletionConstraints {
    // Common
    pub enumerate: Option<Vec<DefaultValue>>,
    pub default: Option<DefaultValue>,

    // Integer OR Float
    pub minimum: Option<DefaultValue>,
    pub maximum: Option<DefaultValue>,
    pub exclusive_minimum: Option<DefaultValue>,
    pub exclusive_maximum: Option<DefaultValue>,
    pub multiple_of: Option<DefaultValue>,
    // String
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,

    // Array
    pub min_items: Option<usize>,
    pub max_items: Option<usize>,
    pub unique_items: Option<bool>,

    // Table
    pub min_keys: Option<usize>,
    pub max_keys: Option<usize>,
    pub key_patterns: Option<Vec<String>>,
    pub additional_keys: bool,
}

impl std::fmt::Display for CompletionConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(enumerate) = &self.enumerate {
            write!(f, "Enumerated Values:\n\n")?;
            for value in enumerate {
                write!(f, "- `{}`\n\n", value)?;
            }
            write!(f, "\n")?;
        }

        if let Some(default) = &self.default {
            write!(f, "Default: `{}`\n\n", default)?;
        }

        if let Some(minimum) = &self.minimum {
            write!(f, "Minimum: `{}`\n\n", minimum)?;
        }

        if let Some(exclusive_minimum) = &self.exclusive_minimum {
            write!(f, "Exclusive Minimum: `{}`\n\n", exclusive_minimum)?;
        }

        if let Some(maximum) = &self.maximum {
            write!(f, "Maximum: `{}`\n\n", maximum)?;
        }

        if let Some(exclusive_maximum) = &self.exclusive_maximum {
            write!(f, "Exclusive Maximum: `{}`\n\n", exclusive_maximum)?;
        }

        if let Some(multiple_of) = &self.multiple_of {
            write!(f, "Multiple of: `{}`\n\n", multiple_of)?;
        }

        if let Some(min_length) = self.min_length {
            write!(f, "Minimum Length: `{}`\n\n", min_length)?;
        }

        if let Some(max_length) = self.max_length {
            write!(f, "Maximum Length: `{}`\n\n", max_length)?;
        }

        if let Some(pattern) = &self.pattern {
            write!(f, "Pattern: `{}`\n\n", pattern)?;
        }

        if let Some(min_items) = self.min_items {
            write!(f, "Minimum Items: `{}`\n\n", min_items)?;
        }

        if let Some(max_items) = self.max_items {
            write!(f, "Maximum Items: `{}`\n\n", max_items)?;
        }

        if self.unique_items.unwrap_or_else(false) {
            write!(f, "Unique Items: `true`\n\n")?;
        }

        if let Some(min_keys) = self.min_keys {
            write!(f, "Minimum Keys: `{}`\n\n", min_keys)?;
        }

        if let Some(max_keys) = self.max_keys {
            write!(f, "Maximum Keys: `{}`\n\n", max_keys)?;
        }

        if let Some(pattern_properties) = &self.pattern_properties {
            content.push_str("Key Patterns:\n\n");
            for pattern_property in pattern_properties.iter() {
                content.push_str(&format!("- `{}`\n\n", pattern_property.key()));
            }
        }

        if self.additional_keys {
            write!(f, "Additional Keys: `true`\n\n")?;
        }

        Ok(())
    }
}

pub enum DefaultValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    OffsetDateTime(String),
    LocalDateTime(String),
    LocalDate(String),
    LocalTime(String),
    Array(Vec<DefaultValue>),
    Table(Vec<(String, DefaultValue)>),
}

impl std::fmt::Display for DefaultValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefaultValue::Boolean(boolean) => write!(f, "{}", boolean),
            DefaultValue::Integer(integer) => write!(f, "{}", integer),
            DefaultValue::Float(float) => write!(f, "{}", float),
            DefaultValue::String(string) => write!(f, "\"{}\"", string.replace("\"", "\\\"")),
            DefaultValue::OffsetDateTime(offset_date_time) => write!(f, "{}", offset_date_time),
            DefaultValue::LocalDateTime(local_date_time) => write!(f, "{}", local_date_time),
            DefaultValue::LocalDate(local_date) => write!(f, "{}", local_date),
            DefaultValue::LocalTime(local_time) => write!(f, "{}", local_time),
            DefaultValue::Array(array) => {
                write!(f, "[")?;
                for (i, value) in array.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            DefaultValue::Table(table) => {
                write!(f, "{{ ")?;
                for (i, (key, value)) in table.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, " }}")
            }
        }
    }
}
