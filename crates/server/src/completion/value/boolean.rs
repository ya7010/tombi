use config::TomlVersion;
use schema_store::{Accessor, BooleanSchema, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::Url;

use crate::completion::{
    CompletionContent, CompletionEdit, CompletionHint, FindCompletionContents,
};

impl FindCompletionContents for BooleanSchema {
    fn find_completion_contents(
        &self,
        _accessors: &Vec<Accessor>,
        _value_schema: Option<&ValueSchema>,
        _toml_version: TomlVersion,
        position: text::Position,
        _keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        _definitions: Option<&SchemaDefinitions>,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent> {
        if let Some(enumerate) = &self.enumerate {
            enumerate
                .iter()
                .map(|value| {
                    let label = value.to_string();
                    let edit = CompletionEdit::new_literal(&label, position, completion_hint);
                    CompletionContent::new_enumerate_value(value.to_string(), edit, schema_url)
                })
                .collect()
        } else {
            type_hint_boolean(position, schema_url, completion_hint)
        }
    }
}

pub fn type_hint_boolean(
    position: text::Position,
    schema_url: Option<&Url>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent> {
    ["true", "false"]
        .into_iter()
        .map(|value| {
            CompletionContent::new_type_hint_value(
                value.to_string(),
                CompletionEdit::new_literal(value, position, completion_hint),
                schema_url,
            )
        })
        .collect()
}
