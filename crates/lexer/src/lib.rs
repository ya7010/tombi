mod cursor;
mod error;
mod lexed;
mod token;

use cursor::Cursor;
pub(self) use error::ErrorKind::*;
pub use error::{Error, ErrorKind};
pub use lexed::Lexed;
use syntax::{SyntaxKind, T};
pub use token::Token;

macro_rules! regex {
    ($($var:ident = $re:expr);+;) => {
        $(
            static $var: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
                regex::Regex::new($re).unwrap()
            });
        )+
    };
}

regex!(
    REGEX_INTEGER_BIN = r"0b[0|1|_]+";
    REGEX_INTEGER_OCT = r"0o[0-7_]+";
    REGEX_INTEGER_HEX = r"0x[0-9A-Fa-f_]+";
    REGEX_INTEGER_DEC = r"[0-9_]+";
    REGEX_FLOAT = r"[0-9_]+(:?(:?\.[0-9_]+)?[eE][+-]?[0-9_]+|\.[0-9_]+)";
    REGEX_OFFSET_DATE_TIME =
        r"\d{4}-\d{2}-\d{2}[Tt ]\d{2}:\d{2}:\d{2}(?:[\.,]\d+)?(?:[Zz]|[+-]\d{2}:\d{2})";
    REGEX_LOCAL_DATE_TIME = r"\d{4}-\d{2}-\d{2}[Tt ]\d{2}:\d{2}:\d{2}(?:[\.,]\d+)?";
    REGEX_LOCAL_DATE = r"\d{4}-\d{2}-\d{2}";
    REGEX_LOCAL_TIME = r"\d{2}:\d{2}:\d{2}(?:[\.,]\d+)?";
);

#[tracing::instrument(level = "debug", skip_all)]
pub fn lex(source: &str) -> Lexed {
    let mut lexed = Lexed::default();
    let mut was_joint = false;
    for res in tokenize(source) {
        match res {
            Ok(token) if token.kind().is_trivia() => was_joint = false,
            _ => {
                if was_joint {
                    lexed.set_joint();
                }
                was_joint = true;
            }
        }
        lexed.push_token_result(res);
    }

    lexed
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Result<Token, crate::Error>> + '_ {
    let mut cursor = Cursor::new(source);
    std::iter::from_fn(move || match cursor.advance_token() {
        Ok(token) => match token.kind() {
            kind if kind != SyntaxKind::EOF => Some(Ok(token)),
            _ => None,
        },
        Err(error) => Some(Err(error)),
    })
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    pub fn advance_token(&mut self) -> Result<Token, crate::Error> {
        if self.bump().is_none() {
            return Ok(Token::eof());
        }
        match self.current() {
            _ if self.is_whitespace() => self.whitespace(),
            _ if self.is_line_break() => self.line_break(),
            '#' => self.line_comment(),
            '"' => {
                if self.matches(r#"""""#) {
                    self.multi_line_basic_string()
                } else {
                    self.basic_string()
                }
            }
            // number
            '0'..='9' => {
                if self.is_datetime() {
                    self.datetime()
                } else if self.is_time() {
                    self.time()
                } else {
                    self.number()
                }
            }
            '\'' => {
                if self.matches("'''") {
                    self.multi_line_literal_string()
                } else {
                    self.literal_string()
                }
            }
            '+' | '-' => {
                self.bump();
                if self.is_keyword("inf") || self.is_keyword("nan") {
                    self.eat_n(2);
                    Ok(Token::new(SyntaxKind::FLOAT, self.pop_span_range()))
                } else if self.current().is_ascii_digit() {
                    self.number()
                } else {
                    self.eat_while(|c| !is_token_separator(c));
                    Err(crate::Error::new(InvalidNumber, self.pop_span_range()))
                }
            }
            '{' => Ok(Token::new(T!('{'), self.pop_span_range())),
            '}' => Ok(Token::new(T!('}'), self.pop_span_range())),
            '[' => Ok(Token::new(T!('['), self.pop_span_range())),
            ']' => Ok(Token::new(T!(']'), self.pop_span_range())),
            ',' => Ok(Token::new(T!(,), self.pop_span_range())),
            '.' => Ok(Token::new(T!(.), self.pop_span_range())),
            '=' => Ok(Token::new(T!(=), self.pop_span_range())),
            'A'..='Z' | 'a'..='z' | '_' => {
                if self.is_keyword("inf") || self.is_keyword("nan") {
                    self.eat_n(2);
                    Ok(Token::new(SyntaxKind::FLOAT, self.pop_span_range()))
                } else if self.is_keyword("true") {
                    self.eat_n(3);
                    Ok(Token::new(SyntaxKind::BOOLEAN, self.pop_span_range()))
                } else if self.is_keyword("false") {
                    self.eat_n(4);
                    Ok(Token::new(SyntaxKind::BOOLEAN, self.pop_span_range()))
                } else {
                    self.key()
                }
            }
            _ => {
                self.eat_while(|c| !is_token_separator(c));
                Err(crate::Error::new(InvalidToken, self.pop_span_range()))
            }
        }
    }

    fn is_whitespace(&self) -> bool {
        is_whitespace(self.current())
    }

    fn whitespace(&mut self) -> Result<Token, crate::Error> {
        self.eat_while(|c| is_whitespace(c));
        Ok(Token::new(SyntaxKind::WHITESPACE, self.pop_span_range()))
    }

    fn line_comment(&mut self) -> Result<Token, crate::Error> {
        assert!(self.current() == '#');

        self.eat_while(|c| !matches!(c, '\n' | '\r'));
        Ok(Token::new(SyntaxKind::COMMENT, self.pop_span_range()))
    }

    fn is_line_break(&self) -> bool {
        is_line_break(self.current())
    }

    fn line_break(&mut self) -> Result<Token, crate::Error> {
        let c = self.current();

        assert!(matches!(c, '\n' | '\r'));
        if self.matches("\r\n") {
            self.eat_n(1);
            2
        } else {
            1
        };

        Ok(Token::new(SyntaxKind::LINE_BREAK, self.pop_span_range()))
    }

    #[inline]
    fn is_keyword(&self, keyword: &str) -> bool {
        self.matches(keyword) && is_token_separator(self.peek(keyword.len()))
    }

    fn is_datetime(&self) -> bool {
        assert!(self.current().is_ascii_digit());
        assert!("2000-01-01".len() == 10);
        REGEX_LOCAL_DATE.is_match(&self.peeks_with_current(10))
    }

    fn datetime(&mut self) -> Result<Token, crate::Error> {
        assert!(self.current().is_ascii_digit());

        let mut pass_local_date_time = false;
        let mut pass_local_date = false;

        let line = self.peek_with_current_while(|c| !is_line_break(c));
        if let Some(m) = REGEX_OFFSET_DATE_TIME.find(&line) {
            if m.end() > 1 {
                self.eat_n(m.end() - 1);
            }

            if is_token_separator(self.peek(1)) {
                return Ok(Token::new(
                    SyntaxKind::OFFSET_DATE_TIME,
                    self.pop_span_range(),
                ));
            }
        } else if let Some(m) = REGEX_LOCAL_DATE_TIME.find(&line) {
            pass_local_date_time = true;

            if m.end() > 1 {
                self.eat_n(m.end() - 1);
            }
            if is_token_separator(self.peek(1)) {
                return Ok(Token::new(
                    SyntaxKind::LOCAL_DATE_TIME,
                    self.pop_span_range(),
                ));
            }
        } else if let Some(m) = REGEX_LOCAL_DATE.find(&line) {
            pass_local_date = true;

            if m.end() > 1 {
                self.eat_n(m.end() - 1);
            }
            if is_token_separator(self.peek(1)) {
                return Ok(Token::new(SyntaxKind::LOCAL_DATE, self.pop_span_range()));
            }
        }

        self.eat_while(|c| !is_token_separator(c));
        if pass_local_date_time {
            Err(crate::Error::new(
                InvalidOffsetDateTime,
                self.pop_span_range(),
            ))
        } else if pass_local_date {
            Err(crate::Error::new(
                InvalidLocalDateTime,
                self.pop_span_range(),
            ))
        } else {
            Err(crate::Error::new(InvalidLocalDate, self.pop_span_range()))
        }
    }

    fn is_time(&self) -> bool {
        assert!(self.current().is_ascii_digit());
        assert!("00:00:00".len() == 8);
        REGEX_LOCAL_TIME.is_match(&self.peeks_with_current(8))
    }

    fn time(&mut self) -> Result<Token, crate::Error> {
        assert!(self.current().is_ascii_digit());

        let line = self.peek_with_current_while(|c| !is_line_break(c));
        if let Some(m) = REGEX_LOCAL_TIME.find(&line) {
            if m.end() > 1 {
                self.eat_n(m.end() - 1);
            }

            Ok(Token::new(SyntaxKind::LOCAL_TIME, self.pop_span_range()))
        } else {
            self.eat_while(|c| !is_line_break(c) && !is_whitespace(c));

            Err(crate::Error::new(InvalidLocalTime, self.pop_span_range()))
        }
    }

    fn number(&mut self) -> Result<Token, crate::Error> {
        let line = self.peek_with_current_while(|c| !is_line_break(c));
        if let Some(m) = REGEX_FLOAT.find(&line) {
            if m.end() > 1 {
                self.eat_n(m.end() - 1);
            }

            if is_token_separator(self.peek(1)) {
                return Ok(Token::new(SyntaxKind::FLOAT, self.pop_span_range()));
            }
        } else if let Some(m) = REGEX_INTEGER_BIN.find(&line) {
            if m.end() > 1 {
                self.eat_n(m.end() - 1);
            }

            if is_token_separator(self.peek(1)) {
                return Ok(Token::new(SyntaxKind::INTEGER_BIN, self.pop_span_range()));
            }
        } else if let Some(m) = REGEX_INTEGER_OCT.find(&line) {
            if m.end() > 1 {
                self.eat_n(m.end() - 1);
            }

            if is_token_separator(self.peek(1)) {
                return Ok(Token::new(SyntaxKind::INTEGER_OCT, self.pop_span_range()));
            }
        } else if let Some(m) = REGEX_INTEGER_HEX.find(&line) {
            if m.end() > 1 {
                self.eat_n(m.end() - 1);
            }
            if is_token_separator(self.peek(1)) {
                return Ok(Token::new(SyntaxKind::INTEGER_HEX, self.pop_span_range()));
            }
        } else if let Some(m) = REGEX_INTEGER_DEC.find(&line) {
            if m.end() > 1 {
                self.eat_n(m.end() - 1);
            }

            if is_token_separator(self.peek(1)) {
                return Ok(Token::new(SyntaxKind::INTEGER_DEC, self.pop_span_range()));
            }
        }

        self.eat_while(|c| !is_token_separator(c));

        Err(crate::Error::new(InvalidNumber, self.pop_span_range()))
    }

    fn basic_string(&mut self) -> Result<Token, crate::Error> {
        self.single_line_string(SyntaxKind::BASIC_STRING, '"', InvalidBasicString)
    }

    fn multi_line_basic_string(&mut self) -> Result<Token, crate::Error> {
        self.multi_line_string(
            SyntaxKind::MULTI_LINE_BASIC_STRING,
            '"',
            InvalidMultilineBasicString,
        )
    }

    fn literal_string(&mut self) -> Result<Token, crate::Error> {
        self.single_line_string(SyntaxKind::LITERAL_STRING, '\'', InvalidLiteralString)
    }

    fn multi_line_literal_string(&mut self) -> Result<Token, crate::Error> {
        self.multi_line_string(
            SyntaxKind::MULTI_LINE_LITERAL_STRING,
            '\'',
            InvalidMultilineLiteralString,
        )
    }

    fn single_line_string(
        &mut self,
        kind: SyntaxKind,
        quote: char,
        error_kind: crate::ErrorKind,
    ) -> Result<Token, crate::Error> {
        assert!(self.current() == quote);

        while let Some(c) = self.bump() {
            match c {
                _ if c == quote => return Ok(Token::new(kind, self.pop_span_range())),
                '\\' if self.peek(1) == quote => {
                    self.bump();
                }
                _ if is_line_break(self.peek(1)) => break,
                _ => (),
            }
        }

        Err(crate::Error::new(error_kind, self.pop_span_range()))
    }

    fn multi_line_string(
        &mut self,
        kind: SyntaxKind,
        quote: char,
        error_kind: crate::ErrorKind,
    ) -> Result<Token, crate::Error> {
        assert!(self.current() == quote && self.peek(1) == quote);

        while let Some(c) = self.bump() {
            match c {
                _ if self.current() == quote && self.peek(1) == quote && self.peek(2) == quote => {
                    self.eat_n(2);
                    return Ok(Token::new(kind, self.pop_span_range()));
                }
                _ => (),
            }
        }

        Err(crate::Error::new(error_kind, self.pop_span_range()))
    }

    fn key(&mut self) -> Result<Token, crate::Error> {
        self.eat_while(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '-'));
        if is_token_separator(self.peek(1)) {
            Ok(Token::new(SyntaxKind::BARE_KEY, self.pop_span_range()))
        } else {
            self.eat_while(|c| !is_token_separator(c));
            Err(crate::Error::new(InvalidKey, self.pop_span_range()))
        }
    }
}

#[inline]
fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t')
}

#[inline]
fn is_line_break(c: char) -> bool {
    matches!(c, '\r' | '\n')
}

#[inline]
fn is_token_separator(c: char) -> bool {
    matches!(
        c,
        '{' | '}' | '[' | ']' | ',' | '.' | '=' | ' ' | '\t' | '\r' | '\n' | '#' | '\0'
    )
}
