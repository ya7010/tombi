#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("{error}")]
    InvalidToken {
        error: syntax::Error,
        span: logos::Span,
    },
}
