#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Range {
    start: text::Position,
    end: text::Position,
}

impl Range {
    #[inline]
    pub fn from_source(source: &str, node: impl ast::AstNode) -> Self {
        let start = text::Position::from_source(source, node.syntax().text_range().start());
        let end = text::Position::from_source(source, node.syntax().text_range().end());
        Self { start, end }
    }

    #[inline]
    pub fn start(&self) -> &text::Position {
        &self.start
    }

    #[inline]
    pub fn end(&self) -> &text::Position {
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
impl From<Range> for tower_lsp::lsp_types::Range {
    fn from(val: Range) -> Self {
        tower_lsp::lsp_types::Range {
            start: val.start.into(),
            end: val.end.into(),
        }
    }
}
