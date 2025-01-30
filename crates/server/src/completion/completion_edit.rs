use tower_lsp::lsp_types::{CompletionTextEdit, InsertReplaceEdit, InsertTextFormat, TextEdit};

use super::CompletionHint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionEdit {
    pub insert_text_format: Option<tower_lsp::lsp_types::InsertTextFormat>,
    pub text_edit: tower_lsp::lsp_types::CompletionTextEdit,
    pub additional_text_edits: Option<Vec<tower_lsp::lsp_types::TextEdit>>,
}

impl CompletionEdit {
    pub fn new_literal(
        value: &str,
        position: text::Position,
        completion_hint: Option<CompletionHint>,
    ) -> Option<Self> {
        match completion_hint {
            Some(
                CompletionHint::DotTrigger { range }
                | CompletionHint::EqualTrigger { range }
                | CompletionHint::SpaceTrigger { range },
            ) => {
                let edit_range = text::Range::new(position, position).into();
                Some(Self {
                    insert_text_format: None,
                    text_edit: CompletionTextEdit::InsertAndReplace(InsertReplaceEdit {
                        new_text: format!(" = {}", value),
                        insert: edit_range,
                        replace: edit_range,
                    }),
                    additional_text_edits: Some(vec![TextEdit {
                        range: range.into(),
                        new_text: "".to_string(),
                    }]),
                })
            }
            _ => None,
        }
    }

    pub fn new_string_literal(
        position: text::Position,
        completion_hint: Option<CompletionHint>,
    ) -> Option<Self> {
        match completion_hint {
            Some(
                CompletionHint::DotTrigger { range }
                | CompletionHint::EqualTrigger { range }
                | CompletionHint::SpaceTrigger { range },
            ) => {
                let edit_range = text::Range::new(position, position).into();
                Some(Self {
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    text_edit: CompletionTextEdit::InsertAndReplace(InsertReplaceEdit {
                        new_text: " = \"$0\"".to_string(),
                        insert: edit_range,
                        replace: edit_range,
                    }),
                    additional_text_edits: Some(vec![TextEdit {
                        range: range.into(),
                        new_text: "".to_string(),
                    }]),
                })
            }
            _ => None,
        }
    }
}
