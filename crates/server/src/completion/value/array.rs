use super::{CompletionHint, FindCompletionItems2};
use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::{CompletionItem, Url};

impl FindCompletionItems2 for document_tree::Array {
    fn find_completion_items2(
        &self,
        _accessors: &[Accessor],
        _value_schema: &ValueSchema,
        _toml_version: TomlVersion,
        _position: text::Position,
        _keys: &[document_tree::Key],
        _schema_url: Option<&Url>,
        _definitions: &SchemaDefinitions,
        _completion_hint: Option<CompletionHint>,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>) {
        (Vec::new(), Vec::new())
    }
}
