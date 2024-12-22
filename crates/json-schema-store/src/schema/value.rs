use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Boolean(b),
            serde_json::Value::Number(n) => {
                if let Some(v) = n.as_i64() {
                    Value::Integer(v)
                } else if let Some(v) = n.as_f64() {
                    Value::Float(v)
                } else {
                    unreachable!()
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(array) => {
                Value::Array(array.into_iter().map(Value::from).collect())
            }
            serde_json::Value::Object(object) => Value::Object(
                object
                    .into_iter()
                    .map(|(k, v)| (k, Value::from(v)))
                    .collect(),
            ),
        }
    }
}
