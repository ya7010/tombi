use text_position::TextPosition;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Range {
    start: TextPosition,
    end: TextPosition,
}

impl Range {
    #[inline]
    pub fn from_source(source: &str, node: impl ast::AstNode) -> Self {
        let start = TextPosition::from_source(source, node.syntax().text_range().start());
        let end = TextPosition::from_source(source, node.syntax().text_range().end());
        Self { start, end }
    }

    #[inline]
    pub fn start(&self) -> &TextPosition {
        &self.start
    }

    #[inline]
    pub fn end(&self) -> &TextPosition {
        &self.end
    }

    pub fn merge(&self, other: &Self) -> Self {
        let start = if self.start < other.start {
            self.start
        } else {
            other.start
        };

        let end = if self.end > other.end {
            self.end
        } else {
            other.end
        };

        Self { start, end }
    }
}

#[cfg(feature = "lsp")]
impl Into<tower_lsp::lsp_types::Range> for Range {
    fn into(self) -> tower_lsp::lsp_types::Range {
        tower_lsp::lsp_types::Range {
            start: self.start.into(),
            end: self.end.into(),
        }
    }
}
