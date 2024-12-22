#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaType {
    Null,
    Boolean,
    Numeric,
    String,
    Array,
    Object,
}
