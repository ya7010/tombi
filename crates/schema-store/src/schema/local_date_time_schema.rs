#[derive(Debug, Default, Clone, PartialEq)]
pub struct LocalDateTimeSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub enumerate: Option<Vec<String>>,
    pub default: Option<String>,
    pub deprecated: Option<bool>,
}

impl LocalDateTimeSchema {
    pub fn new(object: &serde_json::Map<String, serde_json::Value>) -> Self {
        Self {
            title: object
                .get("title")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
            description: object
                .get("description")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
            enumerate: object
                .get("enum")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().map(|v| v.to_string()).collect()),
            default: object
                .get("default")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
            deprecated: object.get("deprecated").and_then(|v| v.as_bool()),
        }
    }

    pub const fn value_type(&self) -> crate::ValueType {
        crate::ValueType::LocalDateTime
    }
}
