use schema_store::get_schema_name;
use tower_lsp::lsp_types::Url;

use super::completion_edit::CompletionEdit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum CompletionPriority {
    DefaultValue = 0,
    #[default]
    Normal = 1,
    TypeHint = 2,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CompletionContent {
    pub label: String,
    pub kind: Option<tower_lsp::lsp_types::CompletionItemKind>,
    pub priority: CompletionPriority,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub schema_url: Option<Url>,
    pub edit: Option<CompletionEdit>,
    pub preselect: Option<bool>,
}

impl CompletionContent {
    pub fn new_enumerate_value(
        label: String,
        edit: Option<CompletionEdit>,
        schema_url: Option<&Url>,
    ) -> Self {
        Self {
            label: label.clone(),
            kind: Some(tower_lsp::lsp_types::CompletionItemKind::VALUE),
            priority: CompletionPriority::Normal,
            detail: Some("enum".to_string()),
            documentation: None,
            schema_url: schema_url.cloned(),
            edit,
            preselect: None,
        }
    }

    pub fn new_default_value(
        label: String,
        edit: Option<CompletionEdit>,
        schema_url: Option<&Url>,
    ) -> Self {
        Self {
            label,
            kind: Some(tower_lsp::lsp_types::CompletionItemKind::VALUE),
            priority: CompletionPriority::DefaultValue,
            detail: Some("default".to_string()),
            documentation: None,
            schema_url: schema_url.cloned(),
            edit,
            preselect: Some(true),
        }
    }

    pub fn new_type_hint_value(label: String, edit: Option<CompletionEdit>) -> Self {
        Self {
            label,
            kind: Some(tower_lsp::lsp_types::CompletionItemKind::VALUE),
            priority: CompletionPriority::TypeHint,
            detail: Some("type hint".to_string()),
            documentation: None,
            schema_url: None,
            edit,
            preselect: None,
        }
    }
}

impl From<CompletionContent> for tower_lsp::lsp_types::CompletionItem {
    fn from(completion_content: CompletionContent) -> Self {
        const SECTION_SEPARATOR: &str = "-----";

        let sorted_text = format!(
            "{}_{}",
            (completion_content.priority as u8),
            &completion_content.label
        );

        let mut schema_text = None;
        if let Some(schema_url) = &completion_content.schema_url {
            if let Some(schema_filename) = get_schema_name(schema_url) {
                schema_text = Some(format!("Schema: [{schema_filename}]({schema_url})\n"));
            }
        }
        let documentation = match completion_content.documentation {
            Some(documentation) => {
                let mut documentation = documentation;
                if let Some(schema_text) = schema_text {
                    documentation.push_str(&format!("\n\n{}\n\n", SECTION_SEPARATOR));
                    documentation.push_str(&schema_text);
                }
                Some(documentation)
            }
            None => schema_text,
        };

        let (insert_text_format, text_edit, additional_text_edits) = match completion_content.edit {
            Some(edit) => (
                edit.insert_text_format,
                Some(edit.text_edit),
                edit.additional_text_edits,
            ),
            None => (None, None, None),
        };

        tower_lsp::lsp_types::CompletionItem {
            label: completion_content.label,
            label_details: match completion_content.priority {
                CompletionPriority::DefaultValue => {
                    Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                        detail: None,
                        description: Some("default value".to_string()),
                    })
                }
                CompletionPriority::TypeHint => {
                    Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                        detail: None,
                        description: Some("type hint".to_string()),
                    })
                }
                CompletionPriority::Normal => {
                    if completion_content.kind
                        == Some(tower_lsp::lsp_types::CompletionItemKind::PROPERTY)
                    {
                        Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                            detail: None,
                            description: completion_content.detail.clone(),
                        })
                    } else {
                        None
                    }
                }
            },
            kind: Some(
                completion_content
                    .kind
                    .unwrap_or(tower_lsp::lsp_types::CompletionItemKind::VALUE),
            ),
            detail: completion_content.detail,
            documentation: documentation.map(|documentation| {
                tower_lsp::lsp_types::Documentation::MarkupContent(
                    tower_lsp::lsp_types::MarkupContent {
                        kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                        value: documentation,
                    },
                )
            }),
            sort_text: Some(sorted_text),
            insert_text_format,
            text_edit,
            additional_text_edits,
            preselect: completion_content.preselect,
            ..Default::default()
        }
    }
}
