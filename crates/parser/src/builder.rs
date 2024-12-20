use syntax::SyntaxKind;

use crate::{output, LexedStr};

pub struct Builder<'a, 'b> {
    pub(crate) lexed: &'a LexedStr<'a>,
    pub(crate) token_index: usize,
    pub(crate) state: State,
    pub(crate) sink: &'b mut dyn FnMut(Step<'_>),
}

impl std::fmt::Debug for Builder<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Builder")
            .field("lexed", &self.lexed)
            .field("token_index", &self.token_index)
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
    pub fn new(lexed: &'a LexedStr<'a>, sink: &'b mut dyn FnMut(Step<'_>)) -> Self {
        Self {
            lexed,
            token_index: 0,
            state: State::PendingEnter,
            sink,
        }
    }

    pub fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        match std::mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(Step::FinishNode),
            State::Normal => (),
        }
        self.eat_trivias();
        self.do_token(kind, n_tokens as usize);
    }

    pub fn enter(&mut self, kind: SyntaxKind) {
        match std::mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => {
                (self.sink)(Step::StartNode { kind });
                // No need to attach trivias to previous node: there is no
                // previous node.
                return;
            }
            State::PendingExit => (self.sink)(Step::FinishNode),
            State::Normal => (),
        }

        self.eat_n_trivias();
        (self.sink)(Step::StartNode { kind });
    }

    pub fn exit(&mut self) {
        match std::mem::replace(&mut self.state, State::PendingExit) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(Step::FinishNode),
            State::Normal => (),
        }
    }

    pub fn eat_trivias(&mut self) {
        while self.token_index < self.lexed.len() {
            let kind = self.lexed.kind(self.token_index);
            if !kind.is_trivia() {
                break;
            }
            self.do_token(kind, 1);
        }
    }

    fn n_trivias(&self) -> usize {
        (self.token_index..self.lexed.len())
            .take_while(|&it| self.lexed.kind(it).is_trivia())
            .count()
    }

    pub fn eat_n_trivias(&mut self) {
        for _ in 0..self.n_trivias() {
            let kind = self.lexed.kind(self.token_index);
            assert!(kind.is_trivia());
            self.do_token(kind, 1);
        }
    }

    pub fn do_token(&mut self, kind: SyntaxKind, n_tokens: usize) {
        let text = &self
            .lexed
            .text_in(self.token_index..self.token_index + n_tokens);
        self.token_index += n_tokens;

        (self.sink)(Step::AddToken { kind, text });
    }
}

#[derive(Debug)]
pub enum Step<'a> {
    AddToken { kind: SyntaxKind, text: &'a str },
    StartNode { kind: SyntaxKind },
    FinishNode,
    Error { error: crate::Error },
}

pub fn intersperse_trivia(
    lexed: &LexedStr,
    output: &crate::Output,
    sink: &mut dyn FnMut(Step<'_>),
) -> bool {
    let mut builder = Builder::new(lexed, sink);

    for event in output.iter() {
        match event {
            output::Step::Token {
                kind,
                n_input_tokens: n_raw_tokens,
            } => builder.token(kind, n_raw_tokens),
            output::Step::Enter { kind } => builder.enter(kind),
            output::Step::Exit => builder.exit(),
            output::Step::Error { error } => {
                (builder.sink)(Step::Error { error });
            }
        }
    }

    match std::mem::replace(&mut builder.state, State::Normal) {
        State::PendingExit => {
            builder.eat_trivias();
            (builder.sink)(Step::FinishNode);
        }
        State::PendingEnter | State::Normal => unreachable!(),
    }

    // is_eof?
    builder.token_index == builder.lexed.len()
}
