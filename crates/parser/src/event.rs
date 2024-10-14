#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Event {
    Start {
        kind: syntax::SyntaxKind,
        forward_parent: Option<u32>,
    },

    Finish,

    /// Produce a single leaf-element.
    /// `n_raw_tokens` is used to glue complex contextual tokens.
    /// For example, lexer tokenizes `>>` as `>`, `>`, and
    /// `n_raw_tokens = 2` is used to produced a single `>>`.
    Token {
        kind: syntax::SyntaxKind,
        n_raw_tokens: u8,
    },
}

impl Event {
    pub(crate) fn tombstone() -> Self {
        Event::Start {
            kind: syntax::SyntaxKind::TOMBSTONE,
            forward_parent: None,
        }
    }
}
