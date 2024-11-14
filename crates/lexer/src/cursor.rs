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

    /// Peeks the next symbol from the input stream without consuming it.
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    /// However, getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    pub fn first(&self) -> char {
        // `.next()` optimizes better than `.nth(0)`
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Peeks the second symbol from the input stream without consuming it.
    pub(crate) fn second(&self) -> char {
        // `.next()` optimizes better than `.nth(1)`
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    /// Peeks the third symbol from the input stream without consuming it.
    pub fn third(&self) -> char {
        // `.next()` optimizes better than `.nth(1)`
        let mut iter = self.chars.clone();
        iter.next();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
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
        while predicate(self.first()) && !self.is_eof() {
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
