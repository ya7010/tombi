pub struct Cursor<'a> {
    /// Iterator over chars. Slightly faster than a &str.
    chars: std::str::Chars<'a>,
    current: char,
    offset: text::Offset,
    token_start: text::Offset,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        let mut chars = input.chars();
        let current = chars.next().unwrap_or(EOF_CHAR);

        Cursor {
            chars: input.chars(),
            current,
            offset: 0,
            token_start: 0,
        }
    }

    #[inline]
    pub(crate) fn current(&self) -> char {
        self.current
    }

    pub fn peek(&self, i: usize) -> char {
        assert!(i != 0, "peek(0) is invalid");

        // `.next()` optimizes better than `.nth(0)`
        self.chars
            .clone()
            .skip(i.saturating_sub(1))
            .next()
            .unwrap_or(EOF_CHAR)
    }

    /// Checks if the charactor at the current position is a expected.
    pub fn matches(&self, expected: &str) -> bool {
        let mut iter = expected.chars();
        if iter.next() != Some(self.current) {
            return false;
        }
        for (i, c) in iter.enumerate() {
            if self.peek(i + 1) != c {
                return false;
            }
        }
        true
    }

    /// Checks if there is nothing more to consume.
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Moves to the next character.
    pub(crate) fn bump(&mut self) -> Option<char> {
        self.offset += 1;
        let c = self.chars.next();
        self.current = c.unwrap_or(EOF_CHAR);
        c
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        // It was tried making optimized version of this for eg. line comments, but
        // LLVM can inline all of this and compile it down to fast iteration over bytes.
        while predicate(self.peek(1)) && !self.is_eof() {
            self.bump();
        }
    }

    #[inline]
    pub(crate) fn span(&mut self) -> text::TextRange {
        let start = self.token_start;
        let end = self.offset;
        self.token_start = self.offset;
        text::TextRange::new(start.into(), end.into())
    }
}
