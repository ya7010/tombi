impl From<crate::Position> for tower_lsp::lsp_types::Position {
    fn from(val: crate::Position) -> Self {
        tower_lsp::lsp_types::Position {
            line: val.line(),
            character: val.column(),
        }
    }
}

impl From<crate::Range> for tower_lsp::lsp_types::Range {
    fn from(val: crate::Range) -> Self {
        tower_lsp::lsp_types::Range {
            start: val.start().into(),
            end: val.end().into(),
        }
    }
}

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
