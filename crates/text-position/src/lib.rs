mod position;

#[cfg(feature = "serde")]
mod serde_impls;

pub use crate::position::TextPosition;

#[cfg(target_pointer_width = "16")]
compile_error!("text-size assumes usize >= u32 and does not work on 16-bit targets");
