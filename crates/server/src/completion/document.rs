use std::ops::Deref;

use super::{CompletionHint, FindCompletionItems2};
use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::Url;

impl FindCompletionItems2 for document_tree::DocumentTree {
    fn find_completion_items2(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: &ValueSchema,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (
        Vec<tower_lsp::lsp_types::CompletionItem>,
        Vec<schema_store::Error>,
    ) {
        self.deref().find_completion_items2(
            accessors,
            value_schema,
            toml_version,
            position,
            keys,
            schema_url,
            definitions,
            completion_hint,
        )
    }
}
