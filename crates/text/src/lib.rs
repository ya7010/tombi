mod features;
mod position;
mod range;
mod relative_position;
mod text_range;
mod text_size;
mod traits;

pub type Offset = u32;
pub type Line = u32;
pub type Column = u32;

pub use crate::{
    position::Position, range::Range, relative_position::RelativePosition, text_range::TextRange,
    text_size::TextSize, traits::TextLen,
};

#[cfg(target_pointer_width = "16")]
compile_error!("text-size assumes usize >= u32 and does not work on 16-bit targets");
