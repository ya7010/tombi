use super::ObjectSchema;

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
    OneOf(Vec<ObjectSchema>),
    AnyOf(Vec<ObjectSchema>),
    AllOf(Vec<ObjectSchema>),
}
