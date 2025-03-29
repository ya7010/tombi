mod date_time;
mod path;

pub use date_time::*;
pub use path::*;

#[macro_export]
macro_rules! toml_text_assert_eq {
    ($actual:expr, $expected:expr) => {
        let expected = format!("{}\n", $expected.trim());
        pretty_assertions::assert_eq!($actual, expected);
    };
}
