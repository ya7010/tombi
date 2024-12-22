use super::ValueSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaType {
    Null,
    Boolean,
    Numeric,
    String,
    Array,
    Object,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaComposition {
    Type(SchemaType),
    OneOf(Vec<ValueSchema>),
    AnyOf(Vec<ValueSchema>),
    AllOf(Vec<ValueSchema>),
}
