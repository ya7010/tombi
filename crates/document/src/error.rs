#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("duplicate key: {key}")]
    DuplicateKey { key: crate::Key },
}
