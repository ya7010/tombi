use syntax::SyntaxKind;

use crate::{step::StrStep, LexedStr};

pub struct Builder<'a, 'b> {
    pub(crate) lexed: &'a LexedStr<'a>,
    pub(crate) pos: usize,
    pub(crate) state: State,
    pub(crate) sink: &'b mut dyn FnMut(StrStep<'_>),
}

impl std::fmt::Debug for Builder<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Builder")
            .field("lexed", &self.lexed)
            .field("pos", &self.pos)
            .field("state", &self.state)
            .finish()
    }
}

#[derive(Debug)]
pub enum State {
    PendingEnter,
    Normal,
    PendingExit,
}

impl<'a, 'b> Builder<'a, 'b> {
    pub fn new(lexed: &'a LexedStr<'a>, sink: &'b mut dyn FnMut(StrStep<'_>)) -> Self {
        Self {
            lexed,
            pos: 0,
            state: State::PendingEnter,
            sink,
        }
    }

    pub fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        match std::mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
        self.eat_trivias();
        self.do_token(kind, n_tokens as usize);
    }

    pub fn enter(&mut self, kind: SyntaxKind) {
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

    pub fn exit(&mut self) {
        match std::mem::replace(&mut self.state, State::PendingExit) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
    }

    pub fn eat_trivias(&mut self) {
        while self.pos < self.lexed.len() {
            let kind = self.lexed.kind(self.pos);
            if !kind.is_trivia() {
                break;
            }
            self.do_token(kind, 1);
        }
    }

    pub fn eat_n_trivias(&mut self, n: usize) {
        for _ in 0..n {
            let kind = self.lexed.kind(self.pos);
            assert!(kind.is_trivia());
            self.do_token(kind, 1);
        }
    }

    pub fn do_token(&mut self, kind: SyntaxKind, n_tokens: usize) {
        let text = &self.lexed.range_text(self.pos..self.pos + n_tokens);
        self.pos += n_tokens;
        (self.sink)(StrStep::Token { kind, text });
    }
}
