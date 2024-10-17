#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parse error")]
    ParseInvalid(Vec<syntax::SyntaxError>),
}
