mod boolean;
mod root;

pub trait Format {
    fn format(&self) -> String;
}
