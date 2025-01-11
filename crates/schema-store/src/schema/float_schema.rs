#[derive(Debug, Default, Clone, PartialEq)]
pub struct FloatSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub exclusive_minimum: Option<f64>,
    pub exclusive_maximum: Option<f64>,
    pub multiple_of: Option<f64>,
    pub default: Option<f64>,
}

impl FloatSchema {
    pub fn new(object: &serde_json::Map<String, serde_json::Value>) -> Self {
        Self {
            title: object
                .get("title")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
            description: object
                .get("description")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
            minimum: object.get("minimum").and_then(|v| v.as_f64()),
            maximum: object.get("maximum").and_then(|v| v.as_f64()),
            exclusive_minimum: object.get("exclusiveMinimum").and_then(|v| v.as_f64()),
            exclusive_maximum: object.get("exclusiveMaximum").and_then(|v| v.as_f64()),
            multiple_of: object.get("multipleOf").and_then(|v| v.as_f64()),
            default: object.get("default").and_then(|v| v.as_f64()),
        }
    }
}
