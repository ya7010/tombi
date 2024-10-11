#[derive(thiserror::Error, Default, Debug, Clone, PartialEq)]
pub enum Error {
    #[default]
    #[error("Invalid token")]
    InvalidToken,
}
