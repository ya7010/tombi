mod arena;
mod error;
pub mod features;
mod parser;
use crate::error::Error;

pub use arena::{ArrayArena, ArrayId, ObjectArena, ObjectId, StrArena, StrId, ValueArena, ValueId};
pub use parser::parse;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(StrId),
    Array(ArrayId),
    Object(ObjectId),
}
