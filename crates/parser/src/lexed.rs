use syntax::SyntaxKind::{self, *};

use crate::builder::{Builder, State};
use crate::{input::Input, output};

#[derive(Debug)]
pub struct LexedStr<'a> {
    pub source: &'a str,
    pub tokens: Vec<lexer::Token>,
    pub errors: Vec<crate::Error>,
}

#[derive(Debug)]
pub enum Step<'a> {
    AddToken { kind: SyntaxKind, text: &'a str },
    StartNode { kind: SyntaxKind },
    FinishNode,
    Error { error: crate::Error },
}

pub fn lex(source: &str) -> LexedStr<'_> {
    let _p = tracing::info_span!("lex").entered();
    LexedStr::new(source)
}

impl<'a> LexedStr<'a> {
    pub fn new(source: &'a str) -> Self {
        let _p = tracing::info_span!("LexedStr::new").entered();
        let mut tokens = Vec::new();
        let mut last_offset = text::Offset::default();
        let mut last_position = text::Position::default();
        let mut errors = Vec::new();

        for result_token in lexer::lex(source).into_iter() {
            let token = match result_token {
                Ok(token) => token,
                Err(error) => {
                    let span_range = (error.span(), error.range());
                    errors.push(error.into());
                    lexer::Token::new(SyntaxKind::INVALID_TOKEN, span_range)
                }
            };
            tokens.push(token);
            last_offset = token.span().end();
            last_position = token.range().end();
        }

        tokens.push(lexer::Token::new(
            EOF,
            (
                text::Span::new(last_offset, text::Offset::new(source.len() as u32)),
                text::Range::new(
                    last_position,
                    last_position + text::RelativePosition::of(&source[last_offset.into()..]),
                ),
            ),
        ));

        Self {
            source,
            tokens,
            errors,
        }
    }

    pub fn as_str(&self) -> &str {
        self.source
    }

    pub fn len(&self) -> usize {
        self.tokens.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self, i: usize) -> SyntaxKind {
        assert!(i < self.len());
        self.tokens[i].kind()
    }

    pub fn text(&self, i: usize) -> &str {
        self.range_text(i..i + 1)
    }

    pub fn range_text(&self, r: std::ops::Range<usize>) -> &str {
        assert!(r.start < r.end && r.end <= self.len());
        let lo = self.tokens[r.start].span().start().into();
        let hi = self.tokens[r.end].span().start().into();
        &self.source[lo..hi]
    }

    pub fn to_input(&self) -> Input {
        let _p = tracing::info_span!("Lexer<'a, SyntaxKind>::to_input").entered();

        let mut res = Input::default();
        let mut was_joint = false;
        for token in self.tokens.iter() {
            let kind = token.kind();
            if kind.is_trivia() {
                was_joint = false
            } else {
                if was_joint {
                    res.was_joint();
                }
                res.push(token.clone());
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
}
