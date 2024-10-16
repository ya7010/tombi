mod boolean;
mod key_value;
mod root;
mod string;

pub trait Format {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String;

    #[allow(unused)]
    fn format_default(&self) -> String {
        self.format(&crate::Context::default())
    }
}
