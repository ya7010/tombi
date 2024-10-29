#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("parse error")]
    ParseInvalid(Vec<ParseError>),
}

#[derive(Debug, Clone, thiserror::Error)]
pub struct ParseError {
    pub message: String,
    pub range: text::TextRange,
    pub text: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {:?} text {:?}",
            self.message, self.range, self.text
        )
    }
}

impl Error {
    pub fn from_syntax_error(source: &str, errors: &[syntax::SyntaxError]) -> Self {
        Self::ParseInvalid(
            errors
                .into_iter()
                .map(|err| ParseError {
                    message: err.message().to_owned(),
                    range: err.range(),
                    text: source[err.range()].to_string(),
                })
                .collect(),
        )
    }
}
