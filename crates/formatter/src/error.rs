#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    Dom(Vec<dom::Error>),
}
