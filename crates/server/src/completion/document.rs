use std::ops::Deref;

use super::{CompletionContent, CompletionHint, FindCompletionItems};
use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::Url;

impl FindCompletionItems for document_tree::DocumentTree {
    fn find_completion_items(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: &ValueSchema,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        self.deref().find_completion_items(
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
