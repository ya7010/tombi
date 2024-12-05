use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct HoverContent {
    pub keys: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub enumerate: Vec<String>,
    pub schema_url: Option<tower_lsp::lsp_types::Url>,
    pub range: text::Range,
}

impl std::fmt::Display for HoverContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.keys)?;
        Ok(())
    }
}

impl HoverContent {
    pub fn into_hover(self) -> tower_lsp::lsp_types::Hover {
        tower_lsp::lsp_types::Hover {
            contents: tower_lsp::lsp_types::HoverContents::Markup(
                tower_lsp::lsp_types::MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: self.to_string(),
                },
            ),
            range: Some(self.range.into()),
        }
    }
}
