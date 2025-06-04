// src/arena.rs
// アリーナ本体とID型

pub mod string;
pub mod value;

pub use string::{StrArena, StrId};
pub use value::{ValueArena, ValueId};
