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
            ValueSchema::Table(table) => table.find_completion_items(accessors, definitions),
            _ => (Vec::new(), Vec::new()),
        }
    }
}
