#[derive(Debug)]
pub struct Error {
    token_index: usize,
    error: syntax::Error,
}

impl Error {
    pub fn new(token_index: usize, error: syntax::Error) -> Self {
        Self { token_index, error }
    }

    pub fn token(&self) -> usize {
        self.token_index
    }

    pub fn msg(&self) -> &str {
        self.error.as_str()
    }
}
