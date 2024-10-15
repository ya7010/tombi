use logos::Logos;
use syntax::SyntaxKind::{self, *};

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

        println!("kind: {:?}", kind);
        println!("start: {:?}", start);
        println!("error: {:?}", error);

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
        let _p = tracing::info_span!("Into<Input> for Lexer<'a, SyntaxKind>::into").entered();

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
        let mut builder = Builder {
            lexed: self,
            pos: 0,
            state: State::PendingEnter,
            sink,
        };

        for event in output.iter() {
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

struct Builder<'a, 'b> {
    lexed: &'a LexedStr<'a>,
    pos: usize,
    state: State,
    sink: &'b mut dyn FnMut(StrStep<'_>),
}

enum State {
    PendingEnter,
    Normal,
    PendingExit,
}

impl Builder<'_, '_> {
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        match std::mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
        self.eat_trivias();
        self.do_token(kind, n_tokens as usize);
    }

    fn enter(&mut self, kind: SyntaxKind) {
        match std::mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => {
                (self.sink)(StrStep::Enter { kind });
                // No need to attach trivias to previous node: there is no
                // previous node.
                return;
            }
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }

        let n_trivias = (self.pos..self.lexed.len())
            .take_while(|&it| self.lexed.kind(it).is_trivia())
            .count();
        let leading_trivias = self.pos..self.pos + n_trivias;
        self.eat_n_trivias(n_trivias);
        // self.eat_n_trivias(n_trivias - n_attached_trivias);
        // (self.sink)(StrStep::Enter { kind });
        // self.eat_n_trivias(n_attached_trivias);
    }

    fn exit(&mut self) {
        match std::mem::replace(&mut self.state, State::PendingExit) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
    }

    fn eat_trivias(&mut self) {
        while self.pos < self.lexed.len() {
            let kind = self.lexed.kind(self.pos);
            if !kind.is_trivia() {
                break;
            }
            self.do_token(kind, 1);
        }
    }

    fn eat_n_trivias(&mut self, n: usize) {
        for _ in 0..n {
            let kind = self.lexed.kind(self.pos);
            assert!(kind.is_trivia());
            self.do_token(kind, 1);
        }
    }

    fn do_token(&mut self, kind: SyntaxKind, n_tokens: usize) {
        let text = &self.lexed.range_text(self.pos..self.pos + n_tokens);
        self.pos += n_tokens;
        (self.sink)(StrStep::Token { kind, text });
    }
}
