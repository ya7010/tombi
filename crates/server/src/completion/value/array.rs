use super::{CompletionHint, FindCompletionItems2};
use schema_store::ValueSchema;
use tower_lsp::lsp_types::CompletionItem;

impl FindCompletionItems2 for document_tree::Array {
    fn find_completion_items2(
        &self,
        _accessors: &[schema_store::Accessor],
        _value_schema: &ValueSchema,
        _toml_version: config::TomlVersion,
        _definitions: &schema_store::SchemaDefinitions,
        _completion_hint: Option<CompletionHint>,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>) {
        (Vec::new(), Vec::new())
    }
}
