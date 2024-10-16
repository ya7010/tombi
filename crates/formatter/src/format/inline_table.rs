use crate::Format;

impl Format for ast::InlineTable {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        unimplemented!("InlineTable::format is not implemented for {:?}", self)
    }
}
