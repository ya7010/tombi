use super::{schema_type::SchemaComposition, value::Value, Referable};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ObjectSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub schema: Option<Referable<SchemaComposition>>,
    pub default: Option<Value>,
    pub enum_values: Vec<Value>,
}
