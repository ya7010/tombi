use text_size::TextSize;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextPosition {
    line: u32,
    column: u32,
}

impl std::fmt::Display for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl TextPosition {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    pub fn from_source(source: &str, offset: TextSize) -> Self {
        let offset = offset.into();
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

impl From<TextSize> for TextPosition {
    fn from(offset: TextSize) -> Self {
        Self::new(0, offset.into())
    }
}
