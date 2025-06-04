mod arena;
pub use arena::{ArrayArena, ArrayId, ObjectArena, ObjectId, StrArena, StrId, ValueArena, ValueId};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(StrId),
    Array(ArrayId),
    Object(ObjectId),
}
