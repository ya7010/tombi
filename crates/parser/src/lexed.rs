use logos::Logos;
use syntax::SyntaxKind::{self, *};

use crate::builder::{Builder, State};
use crate::{input::Input, output::Step, step::StrStep};

#[derive(Debug)]
pub struct LexedStr<'a> {
    pub text: &'a str,
    pub kind: Vec<SyntaxKind>,
    pub start: Vec<u32>,
    pub error: Vec<LexError>,
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
        let mut start = Vec::new();
        let mut error = Vec::new();
        let mut offset = 0;
        while let Some(token) = lexed.next() {
            match token {
                Ok(k) => {
                    kind.push(k);
                    start.push(lexed.span().start as u32)
                }
                Err(err) => error.push(LexError::new(offset, err)),
            }
            offset += 1;
        }
        kind.push(EOF);

        Self {
            text,
            kind,
            start,
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
        let lo = self.start[r.start] as usize;
        let hi = self.start[r.end] as usize;
        &self.text[lo..hi]
    }

    // Naming is hard.
    pub fn text_range(&self, i: usize) -> std::ops::Range<usize> {
        assert!(i < self.len());
        let lo = self.start[i] as usize;
        let hi = self.start[i + 1] as usize;
        lo..hi
    }

    pub fn text_start(&self, i: usize) -> usize {
        assert!(i <= self.len());
        self.start[i] as usize
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
        self.error.iter().map(|it| (it.token() as usize, it.msg()))
    }

    fn push(&mut self, kind: SyntaxKind, offset: usize) {
        self.kind.push(kind);
        self.start.push(offset as u32);
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
        sink: &mut dyn FnMut(StrStep<'_>),
    ) -> bool {
        let mut builder = Builder::new(self, sink);

        for event in output.iter() {
            dbg!(&event);
            match event {
                Step::Token {
                    kind,
                    n_input_tokens: n_raw_tokens,
                } => builder.token(kind, n_raw_tokens),
                Step::Enter { kind } => builder.enter(kind),
                Step::Exit => builder.exit(),
                Step::Error { msg } => {
                    let text_pos = builder.lexed.text_start(builder.pos);
                    (builder.sink)(StrStep::Error { msg, pos: text_pos });
                }
            }
        }
        dbg!(&builder);

        match std::mem::replace(&mut builder.state, State::Normal) {
            State::PendingExit => {
                builder.eat_trivias();
                (builder.sink)(StrStep::Exit);
            }
            State::PendingEnter | State::Normal => unreachable!(),
        }

        // is_eof?
        builder.pos == builder.lexed.len()
    }
}
