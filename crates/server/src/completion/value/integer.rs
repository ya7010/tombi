use config::TomlVersion;
use schema_store::{Accessor, IntegerSchema, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::Url;

use crate::completion::{CompletionHint, FindCompletionItems, FindCompletionItems2};

impl FindCompletionItems2 for document_tree::Integer {
    fn find_completion_items2(
        &self,
        _accessors: &Vec<Accessor>,
        _value_schema: &ValueSchema,
        _toml_version: TomlVersion,
        _position: text::Position,
        _keys: &[document_tree::Key],
        _schema_url: Option<&Url>,
        _definitions: &SchemaDefinitions,
        _completion_hint: Option<CompletionHint>,
    ) -> (
        Vec<tower_lsp::lsp_types::CompletionItem>,
        Vec<schema_store::Error>,
    ) {
        (Vec::with_capacity(0), Vec::with_capacity(0))
    }
}

impl FindCompletionItems for IntegerSchema {
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
                    label: value.to_string(),
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
