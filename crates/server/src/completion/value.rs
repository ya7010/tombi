mod all_of;
mod any_of;
mod one_of;

use super::FindCompletionItems;
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::CompletionItem;

impl FindCompletionItems for ValueSchema {
    fn find_completion_items(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>) {
        match self {
            Self::Table(table) => table.find_completion_items(accessors, definitions),
            Self::AllOf(all_of) => all_of.find_completion_items(accessors, definitions),
            Self::AnyOf(any_of) => any_of.find_completion_items(accessors, definitions),
            Self::OneOf(one_of) => one_of.find_completion_items(accessors, definitions),
            _ => (Vec::new(), Vec::new()),
        }
    }
}
