use super::Format;

impl Format for ast::Root {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        match self
            .items()
            .map(|item| item.format(context))
            .reduce(|acc, item| {
                if item.starts_with('[') {
                    format!("{}\n\n{}", acc, item)
                } else {
                    format!("{}\n{}", acc, item)
                }
            }) {
            Some(it) => it + "\n",
            None => Default::default(),
        }
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
