use super::{CompletionHint, FindCompletionItems};
use schema_store::{Accessor, DocumentSchema, SchemaDefinitions};
use tower_lsp::lsp_types::CompletionItem;

impl FindCompletionItems for DocumentSchema {
    fn find_completion_items(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>) {
        self.table_schema
            .find_completion_items(accessors, definitions, completion_hint)
    }
}
