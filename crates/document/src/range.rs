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
}
