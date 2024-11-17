mod features;
mod offset;
mod position;
mod range;
mod relative_position;
mod span;

type RawTextSize = u32;
pub type Line = RawTextSize;
pub type Column = RawTextSize;

pub use crate::{
    offset::Offset, position::Position, range::Range, relative_position::RelativePosition,
    span::Span,
};

#[cfg(target_pointer_width = "16")]
compile_error!("'text' crate assumes usize >= u32 and does not work on 16-bit targets");
