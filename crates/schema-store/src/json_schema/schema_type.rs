use crate::schema::TableSchema;

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
    OneOf(Vec<Referable<TableSchema>>),
    AnyOf(Vec<Referable<TableSchema>>),
    AllOf(Vec<Referable<TableSchema>>),
}
