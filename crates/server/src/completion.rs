mod document;
mod table;
mod value;

use schema_store::{Accessor, SchemaDefinitions};
use tower_lsp::lsp_types::CompletionItem;

pub trait FindCompletionItems {
    fn find_completion_items(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>);
}
