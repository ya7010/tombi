mod array;
mod array_of_table;
mod comment;
mod inline_table;
mod key_value;
mod literal;
mod root;
mod table;

pub trait Format {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error>;
}
