mod tag;
use crate::highlight::{self, Highlight};
use text_size::TextRange;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HighlightConfig {
    /// Whether to highlight strings
    pub strings: bool,
    /// Whether to highlight punctuation
    pub punctuation: bool,
    /// Whether to highlight operator
    pub operator: bool,
    /// Whether to inject highlights into doc comments
    pub inject_doc_comment: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct HighlightRange {
    pub range: TextRange,
    pub highlight: Highlight,
    pub binding_hash: Option<u64>,
}

pub(crate) fn highlight(config: HighlightConfig, range_to_highlight: Option<TextRange>) {
    let _p = tracing::debug_span!("highlight").entered();
}
