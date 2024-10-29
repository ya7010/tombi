use std::cmp::Ordering;

use crate::TextSize;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Ord)]
pub struct Position {
    line: u32,
    column: u32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.line
                .cmp(&other.line)
                .then_with(|| self.column.cmp(&other.column)),
        )
    }
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    pub fn from_source(source: &str, offset: TextSize) -> Self {
        let offset: usize = offset.into();
        let mut line = 0;
        let mut column = 0;
        for (i, c) in source.char_indices() {
            if i == offset {
                return Self { line, column };
            }
            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        Self { line, column }
    }

    #[inline]
    pub fn line(&self) -> u32 {
        self.line
    }

    #[inline]
    pub fn column(&self) -> u32 {
        self.column
    }
}
