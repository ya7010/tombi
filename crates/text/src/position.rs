use std::cmp::Ordering;

use crate::TextSize;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    line: u32,
    column: u32,
}

impl Position {
    #[inline]
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self { line: 0, column: 0 }
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

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.line
            .cmp(&other.line)
            .then_with(|| self.column.cmp(&other.column))
    }
}

impl PartialOrd for Position {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(u32, u32)> for Position {
    #[inline]
    fn from((line, column): (u32, u32)) -> Self {
        Self::new(line, column)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_position_cmp() {
        use super::Position;

        let p1 = Position::new(1, 2);
        let p2 = Position::new(1, 3);
        let p3 = Position::new(2, 0);

        assert!(p1 < p2);
        assert!(p2 < p3);
        assert!(p1 < p3);
    }
}
