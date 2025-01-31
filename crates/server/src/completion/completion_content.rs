use schema_store::get_schema_name;
use tower_lsp::lsp_types::Url;

use super::completion_edit::CompletionEdit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum CompletionPriority {
    Default = 0,
    Enum = 1,
    #[default]
    Normal = 2,
    TypeHint = 3,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CompletionContent {
    pub label: String,
    pub kind: Option<tower_lsp::lsp_types::CompletionItemKind>,
    pub emoji_icon: Option<char>,
    pub priority: CompletionPriority,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub filter_text: Option<String>,
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
            emoji_icon: None,
            priority: CompletionPriority::Enum,
            detail: Some("enum".to_string()),
            documentation: None,
            filter_text: None,
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
            emoji_icon: None,
            priority: CompletionPriority::Default,
            detail: Some("default".to_string()),
            documentation: None,
            filter_text: None,
            schema_url: schema_url.cloned(),
            edit,
            preselect: Some(true),
        }
    }

    pub fn new_type_hint_value(
        label: impl Into<String>,
        detail: impl Into<String>,
        edit: Option<CompletionEdit>,
        schema_url: Option<&Url>,
    ) -> Self {
        Self {
            label: label.into(),
            kind: Some(tower_lsp::lsp_types::CompletionItemKind::VALUE),
            emoji_icon: Some('🦅'),
            priority: CompletionPriority::TypeHint,
            detail: Some(detail.into()),
            documentation: None,
            filter_text: None,
            schema_url: schema_url.cloned(),
            edit,
            preselect: None,
        }
    }

    pub fn new_type_hint_property(
        label: impl Into<String>,
        edit: Option<CompletionEdit>,
        schema_url: Option<&Url>,
    ) -> Self {
        let priority = CompletionPriority::TypeHint;
        let filter_text = Some(format!("{}_{}", priority as u8, label.into()));

        Self {
            label: "{}".to_string(),
            kind: Some(tower_lsp::lsp_types::CompletionItemKind::PROPERTY),
            emoji_icon: Some('🦅'),
            priority,
            detail: Some("InlineTable".to_string()),
            documentation: None,
            filter_text,
            schema_url: schema_url.cloned(),
            edit,
            preselect: None,
        }
    }

    pub fn new_property(
        label: String,
        detail: Option<String>,
        documentation: Option<String>,
        edit: Option<CompletionEdit>,
        schema_url: Option<&Url>,
    ) -> Self {
        Self {
            label,
            kind: Some(tower_lsp::lsp_types::CompletionItemKind::PROPERTY),
            emoji_icon: None,
            priority: CompletionPriority::Normal,
            detail,
            documentation,
            filter_text: None,
            edit,
            schema_url: schema_url.cloned(),
            preselect: None,
        }
    }
}

impl From<CompletionContent> for tower_lsp::lsp_types::CompletionItem {
    fn from(completion_content: CompletionContent) -> Self {
        const SECTION_SEPARATOR: &str = "-----";

        let sorted_text = format!(
            "{}_{}",
            completion_content.priority as u8, &completion_content.label
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

        let label_details = match completion_content.priority {
            CompletionPriority::Default => Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                detail: None,
                description: Some("default".to_string()),
            }),
            CompletionPriority::Enum => Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                detail: None,
                description: Some("enum".to_string()),
            }),
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
            CompletionPriority::TypeHint => {
                Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                    detail: None,
                    description: Some("type hint".to_string()),
                })
            }
        }
        .map(|mut details| {
            if let Some(emoji_icon) = completion_content.emoji_icon {
                details.description = Some(format!(
                    "{} {}",
                    emoji_icon,
                    details.description.unwrap_or_default()
                ));
            }
            details
        });

        tower_lsp::lsp_types::CompletionItem {
            label: completion_content.label,
            label_details,
            kind: Some(
                completion_content
                    .kind
                    .unwrap_or(tower_lsp::lsp_types::CompletionItemKind::VALUE),
            ),
            detail: completion_content.detail.map(|detail| {
                if let Some(emoji_icon) = completion_content.emoji_icon {
                    format!("{} {}", emoji_icon, detail)
                } else {
                    detail
                }
            }),
            documentation: documentation.map(|documentation| {
                tower_lsp::lsp_types::Documentation::MarkupContent(
                    tower_lsp::lsp_types::MarkupContent {
                        kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                        value: documentation,
                    },
                )
            }),
            sort_text: Some(sorted_text),
            filter_text: completion_content.filter_text,
            insert_text_format,
            text_edit,
            additional_text_edits,
            preselect: completion_content.preselect,
            ..Default::default()
        }
    }
}
