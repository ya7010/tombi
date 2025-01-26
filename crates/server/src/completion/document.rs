use std::ops::Deref;

use super::{CompletionContent, CompletionHint, FindCompletionContents};
use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::Url;

impl FindCompletionContents for document_tree::DocumentTree {
    fn find_completion_contents(
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
        self.deref().find_completion_contents(
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
