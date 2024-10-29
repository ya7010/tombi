use std::cmp::Ordering;

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

    #[inline]
    pub fn line(&self) -> u32 {
        self.line
    }

    #[inline]
    pub fn column(&self) -> u32 {
        self.column
    }
}
