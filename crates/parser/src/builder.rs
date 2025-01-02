use lexer;
use syntax::SyntaxKind;

use crate::output;

pub struct Builder<'a, 'b, 'c> {
    source: &'a str,
    pub(crate) token_index: usize,
    pub(crate) tokens: &'b [lexer::Token],
    pub(crate) state: State,
    pub(crate) sink: &'c mut dyn FnMut(Step<'_>),
}

impl std::fmt::Debug for Builder<'_, '_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Builder")
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

impl<'a, 'b, 'c> Builder<'a, 'b, 'c> {
    pub fn new(
        source: &'a str,
        tokens: &'b [lexer::Token],
        sink: &'c mut dyn FnMut(Step<'_>),
    ) -> Self {
        Self {
            source,
            token_index: 0,
            tokens,
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
        while self.token_index < self.tokens.len() {
            let kind = self.tokens[self.token_index].kind();
            if !kind.is_trivia() {
                break;
            }
            self.do_token(kind, 1);
        }
    }

    fn n_trivias(&self) -> usize {
        (self.token_index..self.tokens.len())
            .take_while(|&it| self.tokens[it].kind().is_trivia())
            .count()
    }

    pub fn eat_n_trivias(&mut self) {
        for _ in 0..self.n_trivias() {
            let kind = self.tokens[self.token_index].kind();
            assert!(kind.is_trivia());
            self.do_token(kind, 1);
        }
    }

    pub fn do_token(&mut self, kind: SyntaxKind, n_tokens: usize) {
        let span = text::Span::new(
            self.tokens[self.token_index].span().start(),
            self.tokens[self.token_index + n_tokens].span().start(),
        );
        let text = &self.source[span];
        // let text = &self.sour
        //     .lexed
        //     .text_in(self.token_index..self.token_index + n_tokens);

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
    source: &str,
    tokens: &[lexer::Token],
    output: &crate::Output,
    sink: &mut dyn FnMut(Step<'_>),
) {
    let mut builder = Builder::new(source, tokens, sink);

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
}
