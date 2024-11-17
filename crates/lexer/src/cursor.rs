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
            offset: Default::default(),
            token_start: Default::default(),
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

    pub fn peeks_with_current(&self, size: usize) -> String {
        let mut iter = self.chars.clone();
        let mut s = String::with_capacity(size + 1);
        s.push(self.current);
        for _ in 0..size {
            if let Some(c) = iter.next() {
                s.push(c);
            } else {
                break;
            }
        }
        s
    }

    pub fn peek_with_current_while(&self, mut predicate: impl FnMut(char) -> bool) -> String {
        let mut iter = self.chars.clone();
        let mut s = String::new();
        s.push(self.current);
        while let Some(c) = iter.next() {
            if predicate(c) {
                s.push(c);
            } else {
                break;
            }
        }
        s
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
        if let Some(c) = self.chars.next() {
            self.offset += text::Offset::new(1);
            self.current = c;
            Some(c)
        } else {
            self.current = EOF_CHAR;
            None
        }
    }

    pub(crate) fn eat_n(&mut self, n: usize) {
        assert!(n > 0);
        for _ in 0..n {
            if self.bump().is_none() {
                break;
            }
        }
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
    pub(crate) fn span(&mut self) -> text::Span {
        let start = self.token_start;
        let end = self.offset;
        self.token_start = self.offset;
        text::Span::new(start.into(), end.into())
    }
}
