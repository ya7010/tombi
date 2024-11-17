mod features;
mod offset;
mod position;
mod range;
mod relative_position;
mod span;
mod traits;

pub type RawOffset = u32;
pub type Line = u32;
pub type Column = u32;

pub use crate::{
    offset::Offset, position::Position, range::Range, relative_position::RelativePosition,
    span::Span, traits::TextLen,
};

#[cfg(target_pointer_width = "16")]
compile_error!("text-size assumes usize >= u32 and does not work on 16-bit targets");
