#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Lexer error: {0:?}")]
    Lexer(tombi_json_lexer::Error),
    #[error("Parser error: {0}")]
    Parser(crate::parser::Error),
    #[error("No value found")]
    NoValue,
}
