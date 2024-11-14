mod cursor;
mod error;
mod lexed;
mod token;

use cursor::Cursor;
pub use error::Error;
pub use lexed::Lexed;
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

        dbg!(self.current());

        let token = match self.current() {
            _ if self.is_whitespace() => self.whitespace(),
            _ if self.is_line_break() => self.line_break(),
            '#' => self.line_comment(),
            '"' => {
                if self.first() == '"' && self.second() == '"' {
                    self.multi_line_basic_string()
                } else {
                    self.basic_string()
                }
            }
            '\'' => {
                if self.first() == '\'' && self.second() == '\'' {
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
        is_line_break(self.current(), self.first())
    }

    fn line_break(&mut self) -> Token {
        let c = self.current();

        assert!(matches!(c, '\n' | '\r'));
        if c == '\r' && self.first() == '\n' {
            self.bump();
            2
        } else {
            1
        };

        Token::new(SyntaxKind::LINE_BREAK, self.span())
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
                '\\' if self.first() == quote => {
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
        assert!(self.current() == quote && self.first() == quote);

        while let Some(c) = self.bump() {
            match c {
                _ if self.current() == quote && self.first() == quote && self.second() == quote => {
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
fn is_line_break(c1: char, c2: char) -> bool {
    c1 == '\n' || (c1 == '\r' && c2 == '\n')
}
