#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("{error}")]
    InvalidToken {
        error: lexer::Error,
        span: logos::Span,
    },
}
