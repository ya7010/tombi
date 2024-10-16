use super::Format;

impl Format for ast::Root {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        self.items()
            .map(|item| item.format(context))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Format for ast::RootItem {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        match self {
            Self::KeyValue(it) => it.format(context),
            _ => unimplemented!("RootItem::format is not implemented for {:?}", self),
        }
    }
}
