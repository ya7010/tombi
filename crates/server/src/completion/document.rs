use std::ops::Deref;

use super::{CompletionHint, FindCompletionItems, FindCompletionItems2};
use config::TomlVersion;
use schema_store::{Accessor, DocumentSchema, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::CompletionItem;

impl FindCompletionItems for DocumentSchema {
    fn find_completion_items(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>) {
        self.table_schema()
            .find_completion_items(accessors, definitions, completion_hint)
    }
}

impl FindCompletionItems2 for document_tree::DocumentTree {
    fn find_completion_items2(
        &self,
        accessors: &[Accessor],
        value_schema: &ValueSchema,
        toml_version: TomlVersion,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>) {
        self.deref().find_completion_items2(
            accessors,
            value_schema,
            toml_version,
            definitions,
            completion_hint,
        )
    }
}
