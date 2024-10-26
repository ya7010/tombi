use crate::{Range, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array {
    value: Vec<Value>,
    range: Range,
}
