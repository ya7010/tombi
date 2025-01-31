use config::TomlVersion;
use schema_store::{Accessor, FloatSchema, SchemaDefinitions, ValueSchema};
use tower_lsp::lsp_types::Url;

use crate::completion::{
    CompletionContent, CompletionEdit, CompletionHint, FindCompletionContents,
};

impl FindCompletionContents for FloatSchema {
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
        let mut completion_items = vec![];

        if let Some(enumerate) = &self.enumerate {
            for item in enumerate {
                let label = item.to_string();
                let edit = CompletionEdit::new_literal(&label, position, completion_hint);
                completion_items.push(CompletionContent::new_enumerate_value(
                    label, edit, schema_url,
                ));
            }
        }

        if let Some(default) = &self.default {
            let label = default.to_string();
            let edit = CompletionEdit::new_literal(&label, position, completion_hint);
            completion_items.push(CompletionContent::new_default_value(
                label, edit, schema_url,
            ));
        }

        if completion_items.is_empty() {
            completion_items.extend(type_hint_float(position, schema_url, completion_hint));
        }

        completion_items
    }
}

pub fn type_hint_float(
    position: text::Position,
    schema_url: Option<&Url>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent> {
    let label = "3.14".to_string();
    let edit = CompletionEdit::new_selectable_literal(&label, position, completion_hint);
    vec![CompletionContent::new_type_hint_value(
        label, edit, schema_url,
    )]
}
