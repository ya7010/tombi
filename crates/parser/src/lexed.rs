use logos::Logos;
use syntax::SyntaxKind::{self, *};
use text::RawOffset;

use crate::builder::{Builder, State};
use crate::{input::Input, output};

#[derive(Debug)]
pub struct LexedStr<'a> {
    pub text: &'a str,
    pub kind: Vec<SyntaxKind>,
    pub start_offsets: Vec<RawOffset>,
    pub error: Vec<LexError>,
}

#[derive(Debug)]
pub enum Step<'a> {
    Token {
        kind: SyntaxKind,
        text: &'a str,
        position: text::Position,
    },
    Enter {
        kind: SyntaxKind,
    },
    Exit,
    Error {
        error: crate::Error,
        position: text::Position,
    },
}

#[derive(Debug)]
pub struct LexError {
    token_index: usize,
    error: syntax::Error,
}

impl LexError {
    pub fn new(token_index: usize, error: syntax::Error) -> Self {
        Self { token_index, error }
    }

    pub fn token(&self) -> usize {
        self.token_index
    }

    pub fn msg(&self) -> &str {
        self.error.as_str()
    }
}

pub fn lex(source: &str) -> LexedStr<'_> {
    let _p = tracing::info_span!("lex").entered();
    LexedStr::new(source)
}

impl<'a> LexedStr<'a> {
    pub fn new(text: &'a str) -> Self {
        let _p = tracing::info_span!("LexedStr::new").entered();
        let mut lexed = SyntaxKind::lexer(text);
        let mut kind = Vec::new();
        let mut start_offsets: Vec<RawOffset> = Vec::new();
        let mut error = Vec::new();
        let mut offset = 0;
        while let Some(token) = lexed.next() {
            match token {
                Ok(k) => {
                    kind.push(k);
                    start_offsets.push(lexed.span().start as RawOffset);
                }
                Err(err) => error.push(LexError::new(offset, err)),
            }
            offset += 1;
        }
        kind.push(EOF);
        start_offsets.push(text.len() as RawOffset);

        Self {
            text,
            kind,
            start_offsets,
            error,
        }
    }

    pub fn single_token(text: &'a str) -> Option<Result<SyntaxKind, syntax::Error>> {
        if text.is_empty() {
            return None;
        }

        SyntaxKind::lexer(text).next()
    }

    pub fn as_str(&self) -> &str {
        self.text
    }

    pub fn len(&self) -> usize {
        self.kind.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self, i: usize) -> SyntaxKind {
        assert!(i < self.len());
        self.kind[i]
    }

    pub fn text(&self, i: usize) -> &str {
        self.range_text(i..i + 1)
    }

    pub fn range_text(&self, r: std::ops::Range<usize>) -> &str {
        assert!(r.start < r.end && r.end <= self.len());
        let lo = self.start_offsets[r.start] as usize;
        let hi = self.start_offsets[r.end] as usize;
        &self.text[lo..hi]
    }

    // Naming is hard.
    pub fn text_range(&self, i: usize) -> std::ops::Range<usize> {
        assert!(i < self.len());
        let lo = self.start_offsets[i] as usize;
        let hi = self.start_offsets[i + 1] as usize;
        lo..hi
    }

    pub fn text_start_offset(&self, i: usize) -> RawOffset {
        assert!(i <= self.len());
        self.start_offsets[i]
    }

    pub fn text_start_position(&self, i: usize) -> text::Position {
        text::Position::from_source(&self.text, (self.text_start_offset(i)).into())
    }

    pub fn text_len(&self, i: usize) -> usize {
        assert!(i < self.len());
        let r = self.text_range(i);
        r.end - r.start
    }

    pub fn error(&self, i: usize) -> Option<&str> {
        assert!(i < self.len());
        let err = self
            .error
            .binary_search_by_key(&(i as u32), |e| e.token() as u32)
            .ok()?;
        Some(self.error[err].msg())
    }

    pub fn errors(&self) -> impl Iterator<Item = (usize, &str)> + '_ {
        self.error.iter().map(|it| (it.token(), it.msg()))
    }

    pub fn to_input(&self) -> Input {
        let _p = tracing::info_span!("Lexer<'a, SyntaxKind>::to_input").entered();

        let mut res = Input::default();
        let mut was_joint = false;
        for kind in self.kind.iter() {
            if kind.is_trivia() {
                was_joint = false
            } else {
                if was_joint {
                    res.was_joint();
                }
                res.push(*kind);
                was_joint = true;
            }
        }
        res
    }

    pub fn intersperse_trivia(
        &self,
        output: &crate::Output,
        sink: &mut dyn FnMut(Step<'_>),
    ) -> bool {
        let mut builder = Builder::new(self, sink);

        for event in output.iter() {
            match event {
                output::Step::Token {
                    kind,
                    n_input_tokens: n_raw_tokens,
                } => builder.token(kind, n_raw_tokens),
                output::Step::Enter { kind } => builder.enter(kind),
                output::Step::Exit => builder.exit(),
                output::Step::Error { error } => {
                    let start_position = builder.lexed.text_start_position(builder.token_index);
                    (builder.sink)(Step::Error {
                        error,
                        position: start_position,
                    });
                }
            }
        }

        match std::mem::replace(&mut builder.state, State::Normal) {
            State::PendingExit => {
                builder.eat_trivias();
                (builder.sink)(Step::Exit);
            }
            State::PendingEnter | State::Normal => unreachable!(),
        }

        // is_eof?
        builder.token_index == builder.lexed.len()
    }
}
