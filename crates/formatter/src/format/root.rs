use super::Format;

impl Format for ast::Root {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        self.items()
            .map(|item| item.format(context))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl Format for ast::RootItem {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        match self {
            Self::KeyValue(it) => it.format(context),
            Self::Table(it) => it.format(context),
            Self::ArrayOfTable(it) => it.format(context),
        }
    }
}
