use schema_store::StringSchema;

use crate::completion::{FindCompletionItems, FindCompletionItems2};

impl FindCompletionItems2 for document_tree::String {
    fn find_completion_items2(
        &self,
        _accessors: &[schema_store::Accessor],
        _value_schema: &schema_store::ValueSchema,
        _toml_version: config::TomlVersion,
        _definitions: &schema_store::SchemaDefinitions,
        _completion_hint: Option<crate::completion::CompletionHint>,
    ) -> (
        Vec<tower_lsp::lsp_types::CompletionItem>,
        Vec<schema_store::Error>,
    ) {
        (Vec::with_capacity(0), Vec::with_capacity(0))
    }
}

impl FindCompletionItems for StringSchema {
    fn find_completion_items(
        &self,
        _accessors: &[schema_store::Accessor],
        _definitions: &schema_store::SchemaDefinitions,
        _completion_hint: Option<crate::completion::CompletionHint>,
    ) -> (
        Vec<tower_lsp::lsp_types::CompletionItem>,
        Vec<schema_store::Error>,
    ) {
        if let Some(enumerate) = &self.enumerate {
            let items = enumerate
                .iter()
                .map(|value| tower_lsp::lsp_types::CompletionItem {
                    label: format!("\"{value}\""),
                    kind: Some(tower_lsp::lsp_types::CompletionItemKind::VALUE),
                    ..Default::default()
                })
                .collect();
            return (items, Vec::with_capacity(0));
        } else {
            (Vec::with_capacity(0), Vec::with_capacity(0))
        }
    }
}
