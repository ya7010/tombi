use tower_lsp::lsp_types::{CompletionTextEdit, InsertTextFormat, TextEdit};

use super::CompletionHint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionEdit {
    pub insert_text_format: Option<tower_lsp::lsp_types::InsertTextFormat>,
    pub text_edit: tower_lsp::lsp_types::CompletionTextEdit,
    pub additional_text_edits: Option<Vec<tower_lsp::lsp_types::TextEdit>>,
}

impl CompletionEdit {
    pub fn new_literal(
        label: &str,
        position: text::Position,
        completion_hint: Option<CompletionHint>,
    ) -> Option<Self> {
        match completion_hint {
            Some(
                CompletionHint::DotTrigger { range, .. }
                | CompletionHint::EqualTrigger { range, .. }
                | CompletionHint::SpaceTrigger { range, .. },
            ) => Some(Self {
                insert_text_format: None,
                text_edit: CompletionTextEdit::Edit(TextEdit {
                    new_text: format!(" = {}", label),
                    range: text::Range::at(position).into(),
                }),
                additional_text_edits: Some(vec![TextEdit {
                    range: range.into(),
                    new_text: "".to_string(),
                }]),
            }),
            _ => None,
        }
    }

    pub fn new_selectable_literal(
        label: &str,
        position: text::Position,
        completion_hint: Option<CompletionHint>,
    ) -> Option<Self> {
        match completion_hint {
            Some(
                CompletionHint::DotTrigger { range, .. }
                | CompletionHint::EqualTrigger { range, .. }
                | CompletionHint::SpaceTrigger { range, .. },
            ) => Some(Self {
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                text_edit: CompletionTextEdit::Edit(TextEdit {
                    new_text: format!(" = ${{1:{}}}", label),
                    range: text::Range::at(position).into(),
                }),
                additional_text_edits: Some(vec![TextEdit {
                    range: range.into(),
                    new_text: "".to_string(),
                }]),
            }),
            _ => None,
        }
    }

    pub fn new_string_literal(
        position: text::Position,
        completion_hint: Option<CompletionHint>,
    ) -> Option<Self> {
        match completion_hint {
            Some(
                CompletionHint::DotTrigger { range, .. }
                | CompletionHint::EqualTrigger { range, .. }
                | CompletionHint::SpaceTrigger { range, .. },
            ) => Some(Self {
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                text_edit: CompletionTextEdit::Edit(TextEdit {
                    new_text: " = \"$0\"".to_string(),
                    range: text::Range::at(position).into(),
                }),
                additional_text_edits: Some(vec![TextEdit {
                    range: range.into(),
                    new_text: "".to_string(),
                }]),
            }),
            Some(CompletionHint::InTableHeader) | None => None,
        }
    }

    pub fn new_array_literal(
        position: text::Position,
        completion_hint: Option<CompletionHint>,
    ) -> Option<Self> {
        match completion_hint {
            Some(
                CompletionHint::DotTrigger { range, .. }
                | CompletionHint::EqualTrigger { range, .. }
                | CompletionHint::SpaceTrigger { range, .. },
            ) => Some(Self {
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                text_edit: CompletionTextEdit::Edit(TextEdit {
                    new_text: " = [$0]".to_string(),
                    range: text::Range::at(position).into(),
                }),
                additional_text_edits: Some(vec![TextEdit {
                    range: range.into(),
                    new_text: "".to_string(),
                }]),
            }),
            Some(CompletionHint::InTableHeader) | None => None,
        }
    }

    pub fn new_propery(
        property_name: &str,
        position: text::Position,
        completion_hint: Option<CompletionHint>,
    ) -> Option<Self> {
        match completion_hint {
            Some(
                CompletionHint::SpaceTrigger { range, .. }
                | CompletionHint::EqualTrigger { range, .. },
            ) => Some(Self {
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                text_edit: CompletionTextEdit::Edit(TextEdit {
                    new_text: format!(" = {{ {property_name}$1 }}"),
                    range: text::Range::at(position).into(),
                }),
                additional_text_edits: Some(vec![TextEdit {
                    range: range.into(),
                    new_text: "".to_string(),
                }]),
            }),
            Some(CompletionHint::DotTrigger { range, .. }) => Some(Self {
                insert_text_format: None,
                text_edit: CompletionTextEdit::Edit(TextEdit {
                    new_text: format!(".{property_name}"),
                    range: text::Range::at(position).into(),
                }),
                additional_text_edits: Some(vec![TextEdit {
                    range: range.into(),
                    new_text: "".to_string(),
                }]),
            }),
            Some(CompletionHint::InTableHeader) | None => None,
        }
    }
}
