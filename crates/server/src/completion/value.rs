mod all_of;
mod any_of;
mod one_of;

use super::{CompletionHint, FindCompletionItems};
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::CompletionItem;

impl FindCompletionItems for ValueSchema {
    fn find_completion_items(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>) {
        match self {
            Self::Table(table) => {
                table.find_completion_items(accessors, definitions, completion_hint)
            }
            Self::AllOf(all_of) => {
                all_of.find_completion_items(accessors, definitions, completion_hint)
            }
            Self::AnyOf(any_of) => {
                any_of.find_completion_items(accessors, definitions, completion_hint)
            }
            Self::OneOf(one_of) => {
                one_of.find_completion_items(accessors, definitions, completion_hint)
            }
            _ => (Vec::new(), Vec::new()),
        }
    }
}
