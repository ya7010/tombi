use crate::schema::ObjectSchema;

use super::Referable;

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
    OneOf(Vec<Referable<ObjectSchema>>),
    AnyOf(Vec<Referable<ObjectSchema>>),
    AllOf(Vec<Referable<ObjectSchema>>),
}
