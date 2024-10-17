mod array;
mod array_of_table;
mod inline_table;
mod key_value;
mod literal;
mod root;
mod table;

pub trait Format {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String;

    #[allow(unused)]
    fn format_default(&self) -> String {
        self.format(&crate::Context::default())
    }
}
