mod cursor;
mod error;
mod lexed;
mod token;

use cursor::Cursor;
pub use error::Error;
pub use lexed::Lexed;
use regex_macro::regex;
use syntax::{SyntaxKind, T};
pub use token::Token;

#[tracing::instrument(level = "debug", skip_all)]
pub fn lex(source: &str) -> Lexed {
    let _p = tracing::info_span!("lex").entered();
    Lexed::new(source)
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(source);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != SyntaxKind::EOF {
            Some(token)
        } else {
            None
        }
    })
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    pub fn advance_token(&mut self) -> Token {
        if self.bump().is_none() {
            return Token::eof();
        }

        let token = match self.current() {
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
                    self.integer()
                }
            }
            '\'' => {
                if self.matches("'''") {
                    self.multi_line_literal_string()
                } else {
                    self.literal_string()
                }
            }
            '{' => Token::new(T!('{'), self.span()),
            '}' => Token::new(T!('}'), self.span()),
            '[' => Token::new(T!('['), self.span()),
            ']' => Token::new(T!(']'), self.span()),
            ',' => Token::new(T!(,), self.span()),
            '.' => Token::new(T!(.), self.span()),
            '=' => Token::new(T!(=), self.span()),
            _ => Token::new(SyntaxKind::INVALID_TOKEN, self.span()),
            // _ => std::process::exit(1),
        };

        token
    }

    fn is_whitespace(&self) -> bool {
        is_whitespace(self.current())
    }

    fn whitespace(&mut self) -> Token {
        self.eat_while(|c| matches!(c, ' ' | '\t'));
        Token::new(SyntaxKind::WHITESPACE, self.span())
    }

    fn line_comment(&mut self) -> Token {
        assert!(self.current() == '#');

        self.eat_while(|c| !matches!(c, '\n' | '\r'));
        Token::new(SyntaxKind::COMMENT, self.span())
    }

    fn is_line_break(&self) -> bool {
        is_line_break(self.current())
    }

    fn line_break(&mut self) -> Token {
        let c = self.current();

        assert!(matches!(c, '\n' | '\r'));
        if self.matches("\r\n") {
            self.bump();
            2
        } else {
            1
        };

        Token::new(SyntaxKind::LINE_BREAK, self.span())
    }

    fn is_datetime(&self) -> bool {
        assert!(self.current().is_ascii_digit());
        assert!("2000-01-01".len() == 10);
        regex!(r"\d{4}-\d{2}-\d{2}").is_match(&self.peeks_with_current(10))
    }

    fn datetime(&mut self) -> Token {
        assert!(self.current().is_ascii_digit());

        let line = self.peek_with_current_while(|c| !is_line_break(c));
        if let Some(m) = regex!(
            r#"\d{4}-\d{2}-\d{2}[Tt ]\d{2}:\d{2}:\d{2}(?:[\.,]\d+)?(?:[Zz]|[+-]\d{2}:\d{2})"#
        )
        .find(&line)
        {
            for _ in 0..m.end() {
                self.bump();
            }
            Token::new(SyntaxKind::OFFSET_DATE_TIME, self.span())
        } else if let Some(m) =
            regex!(r"\d{4}-\d{2}-\d{2}(?:T|t| )\d{2}:\d{2}:\d{2}(?:[\.,]\d+)?").find(&line)
        {
            for _ in 0..m.end() {
                self.bump();
            }
            Token::new(SyntaxKind::LOCAL_DATE_TIME, self.span())
        } else if let Some(m) = regex!(r"\d{4}-\d{2}-\d{2}").find(&line) {
            for _ in 0..m.end() {
                self.bump();
            }
            Token::new(SyntaxKind::LOCAL_DATE, self.span())
        } else {
            self.eat_while(|c| !is_line_break(c) && !is_whitespace(c) && !is_comment(c));
            Token::new(SyntaxKind::INVALID_TOKEN, self.span())
        }
    }

    fn is_time(&self) -> bool {
        assert!(self.current().is_ascii_digit());
        assert!("00:00:00".len() == 8);
        regex!(r"\d{2}:\d{2}:\d{2}").is_match(&self.peeks_with_current(8))
    }

    fn time(&mut self) -> Token {
        assert!(self.current().is_ascii_digit());

        let line = self.peek_with_current_while(|c| !is_line_break(c));
        if let Some(m) = regex!(r"\d{2}:\d{2}:\d{2}(?:[\.,]\d+)?").find(&line) {
            for _ in 0..m.end() {
                self.bump();
            }
            Token::new(SyntaxKind::LOCAL_TIME, self.span())
        } else {
            self.eat_while(|c| !is_line_break(c) && !is_whitespace(c));
            Token::new(SyntaxKind::INVALID_TOKEN, self.span())
        }
    }

    fn integer(&mut self) -> Token {
        assert!(self.current().is_ascii_digit());

        self.eat_while(|c| c.is_ascii_digit());

        Token::new(SyntaxKind::INTEGER_DEC, self.span())
    }

    fn basic_string(&mut self) -> Token {
        self.single_line_string(SyntaxKind::BASIC_STRING, '"')
    }

    fn multi_line_basic_string(&mut self) -> Token {
        self.multi_line_string(SyntaxKind::MULTI_LINE_BASIC_STRING, '"')
    }

    fn literal_string(&mut self) -> Token {
        self.single_line_string(SyntaxKind::LITERAL_STRING, '\'')
    }

    fn multi_line_literal_string(&mut self) -> Token {
        self.multi_line_string(SyntaxKind::MULTI_LINE_LITERAL_STRING, '\'')
    }

    fn single_line_string(&mut self, kind: SyntaxKind, quote: char) -> Token {
        assert!(self.current() == quote);

        while let Some(c) = self.bump() {
            match c {
                _ if c == quote => return Token::new(kind, self.span()),
                '\\' if self.peek(1) == quote => {
                    self.bump();
                }
                _ if self.is_line_break() => {
                    return Token::new(SyntaxKind::INVALID_TOKEN, self.span());
                }
                _ => (),
            }
        }

        Token::new(SyntaxKind::INVALID_TOKEN, self.span())
    }

    fn multi_line_string(&mut self, kind: SyntaxKind, quote: char) -> Token {
        assert!(self.current() == quote && self.peek(1) == quote);

        while let Some(c) = self.bump() {
            match c {
                _ if self.current() == quote && self.peek(1) == quote && self.peek(2) == quote => {
                    self.bump();
                    self.bump();
                    return Token::new(kind, self.span());
                }
                _ => (),
            }
        }

        Token::new(SyntaxKind::INVALID_TOKEN, self.span())
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
fn is_comment(c: char) -> bool {
    matches!(c, '#')
}
