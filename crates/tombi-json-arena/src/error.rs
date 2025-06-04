use tombi_json_lexer::Error as LexerError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Lexer error: {0:?}")]
    Lexer(Vec<LexerError>),
    #[error("Parse error: {0}")]
    Parse(String),
}
