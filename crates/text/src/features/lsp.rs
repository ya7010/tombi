#[cfg(feature = "lsp")]
impl Into<tower_lsp::lsp_types::Position> for crate::Position {
    fn into(self) -> tower_lsp::lsp_types::Position {
        tower_lsp::lsp_types::Position {
            line: self.line(),
            character: self.column(),
        }
    }
}

#[cfg(feature = "lsp")]
impl crate::TextSize {
    pub fn from_source(source: &str, position: tower_lsp::lsp_types::Position) -> Self {
        let mut line = 0;
        let mut column = 0;
        let mut offset = 0;
        for (i, c) in source.char_indices() {
            if line == position.line && column == position.character {
                return Self::new(offset as u32);
            }
            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
            offset = i + c.len_utf8();
        }
        Self::new(offset as u32)
    }
}
