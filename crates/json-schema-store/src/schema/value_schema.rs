use super::SchemaType;

#[derive(Debug, Clone, Default)]
pub struct ValueSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub types: Vec<SchemaType>,
    pub default: Option<String>,
    pub enum_values: Vec<String>,
}
