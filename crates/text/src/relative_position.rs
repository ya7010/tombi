use std::cmp::Ordering;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RelativePosition {
    line: u32,
    column: u32,
}

impl RelativePosition {
    #[inline]
    pub const fn zero() -> Self {
        Self { line: 0, column: 0 }
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

impl Ord for RelativePosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.line
            .cmp(&other.line)
            .then_with(|| self.column.cmp(&other.column))
    }
}

impl PartialOrd for RelativePosition {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(u32, u32)> for RelativePosition {
    #[inline]
    fn from((line, column): (u32, u32)) -> Self {
        Self { line, column }
    }
}

impl From<&str> for RelativePosition {
    #[inline]
    fn from(text: &str) -> Self {
        let mut line = 0;
        let mut column = 0;
        for c in text.chars() {
            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        Self { line, column }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", (0, 0))]
    #[case("a", (0, 1))]
    #[case("abc\ndef\nghi", (2, 3))]
    #[case("abc\r\ndef\r\nghi", (2, 3))]
    fn test_position(#[case] source: &str, #[case] expected: (u32, u32)) {
        assert_eq!(RelativePosition::from(source), expected.into());
    }
}
