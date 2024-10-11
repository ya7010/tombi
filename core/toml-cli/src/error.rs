#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Formatter(#[from] formatter::Error),
}
