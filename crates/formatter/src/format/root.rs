use ast::{Root, RootItem};

use super::Format;

impl Format for Root {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        self.items()
            .map(|item| item.format(context))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Format for RootItem {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        match self {
            RootItem::KeyValue(it) => it.format(context),
            _ => unimplemented!("RootItem::format is not implemented for {:?}", self),
        }
    }
}
