use config::TomlVersion;
use schema_store::{Accessor, SchemaDefinitions, StringSchema, ValueSchema};
use tower_lsp::lsp_types::Url;

use crate::completion::{
    completion_kind::CompletionKind, CompletionContent, CompletionEdit, CompletionHint,
    FindCompletionContents,
};

impl FindCompletionContents for StringSchema {
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
                let label = format!("\"{item}\"");
                let edit = CompletionEdit::new_literal(&label, position, completion_hint);
                completion_items.push(CompletionContent::new_enumerate_value(
                    CompletionKind::String,
                    label,
                    edit,
                    schema_url,
                ));
            }
        }

        if let Some(default) = &self.default {
            let label = format!("\"{default}\"");
            let edit = CompletionEdit::new_literal(&label, position, completion_hint);
            completion_items.push(CompletionContent::new_default_value(
                CompletionKind::String,
                label,
                edit,
                schema_url,
            ));
        }

        if completion_items.is_empty() {
            completion_items.extend(type_hint_string(position, schema_url, completion_hint));
        }

        completion_items
    }
}

pub fn type_hint_string(
    position: text::Position,
    schema_url: Option<&Url>,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent> {
    [('\"', "BasicString"), ('\'', "LiteralString")]
        .into_iter()
        .map(|(quote, detail)| {
            CompletionContent::new_type_hint_string(
                CompletionKind::String,
                quote,
                detail,
                CompletionEdit::new_string_literal(quote, position, completion_hint),
                schema_url,
            )
        })
        .collect()
}
