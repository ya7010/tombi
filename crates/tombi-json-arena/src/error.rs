#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Lexer error: {0:?}")]
    Lexer(tombi_json_lexer::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}
