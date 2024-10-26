#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("duplicate key: {key}")]
    DuplicateKey { key: crate::Key },
}
