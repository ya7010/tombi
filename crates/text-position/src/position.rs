use std::cmp::Ordering;

use text_size::TextSize;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Ord)]
pub struct TextPosition {
    line: u32,
    column: u32,
    offset: usize,
}

impl std::fmt::Display for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
impl PartialOrd for TextPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.offset.cmp(&other.offset))
    }
}

impl TextPosition {
    pub fn from_source(source: &str, offset: TextSize) -> Self {
        let offset = offset.into();
        let mut line = 0;
        let mut column = 0;
        for (i, c) in source.char_indices() {
            if i == offset {
                return Self {
                    line,
                    column,
                    offset,
                };
            }
            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        Self {
            line,
            column,
            offset,
        }
    }

    #[inline]
    pub fn line(&self) -> u32 {
        self.line
    }

    #[inline]
    pub fn column(&self) -> u32 {
        self.column
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[cfg(feature = "lsp")]
impl Into<tower_lsp::lsp_types::Position> for TextPosition {
    fn into(self) -> tower_lsp::lsp_types::Position {
        tower_lsp::lsp_types::Position {
            line: self.line,
            character: self.column,
        }
    }
}
