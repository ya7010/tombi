#[derive(Debug, Default, Clone, PartialEq)]
pub struct LocalDateTimeSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<String>,
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
            default: object
                .get("default")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
        }
    }
}
