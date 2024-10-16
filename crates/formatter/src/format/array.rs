use crate::Format;

impl Format for ast::Array {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        unimplemented!("Array::format is not implemented for {:?}", self)
    }
}
