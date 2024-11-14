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
            '{' => Token::new(T!('{'), self.span()),
            '}' => Token::new(T!('}'), self.span()),
            '[' => Token::new(T!('['), self.span()),
            ']' => Token::new(T!(']'), self.span()),
            ',' => Token::new(T!(,), self.span()),
            '=' => Token::new(T!(=), self.span()),
            _ => Token::new(SyntaxKind::INVALID_TOKEN, self.span()),
            // _ => std::process::exit(1),
        };

        token
    }

    pub fn is_whitespace(&self) -> bool {
        is_whitespace(self.current())
    }

    pub fn whitespace(&mut self) -> Token {
        self.eat_while(|c| matches!(c, ' ' | '\t'));
        Token::new(SyntaxKind::WHITESPACE, self.span())
    }

    pub fn line_comment(&mut self) -> Token {
        assert!(self.current() == '#');

        dbg!((self.current(), self.first(), self.second()));
        self.eat_while(|c| !matches!(c, '\n' | '\r'));
        dbg!((self.current(), self.first(), self.second()));

        Token::new(SyntaxKind::COMMENT, self.span())
    }

    fn is_line_break(&self) -> bool {
        is_line_break(self.current(), self.first())
    }

    pub fn line_break(&mut self) -> Token {
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

    pub fn basic_string(&mut self) -> Token {
        assert!(self.current() == '"' && self.first() != '"');

        while let Some(c) = self.bump() {
            match c {
                '"' => return Token::new(SyntaxKind::BASIC_STRING, self.span()),
                '\\' if self.first() == '"' => {
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

    pub fn multi_line_basic_string(&mut self) -> Token {
        assert!(self.current() == '"' && self.first() == '"' && self.second() == '"');

        while let Some(c) = self.bump() {
            match c {
                '"' if self.first() == '"' && self.second() == '"' => {
                    self.bump();
                    self.bump();
                    return Token::new(SyntaxKind::MULTI_LINE_BASIC_STRING, self.span());
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
