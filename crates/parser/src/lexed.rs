use syntax::SyntaxKind::{self, *};

use crate::input::Input;

#[derive(Debug)]
pub struct LexedStr<'a> {
    pub source: &'a str,
    pub tokens: Vec<lexer::Token>,
    pub errors: Vec<crate::Error>,
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

        for result_token in lexer::lex(source) {
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

    pub fn text_in(&self, r: std::ops::Range<usize>) -> &str {
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
                res.push(*token);
                was_joint = true;
            }
        }
        res
    }
}
