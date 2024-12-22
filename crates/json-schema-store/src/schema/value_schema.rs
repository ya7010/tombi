use super::{schema_type::SchemaComposition, value::Value};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValueSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub types: Option<SchemaComposition>,
    pub default: Option<Value>,
    pub enum_values: Vec<Value>,
}
