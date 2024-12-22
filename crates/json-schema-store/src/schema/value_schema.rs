use super::{default_value::DefaultValue, schema_type::SchemaComposition};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValueSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub types: Option<SchemaComposition>,
    pub default: Option<DefaultValue>,
    pub enum_values: Vec<String>,
}
