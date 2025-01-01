#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to read {path:?}")]
    ReadFailed { path: std::path::PathBuf },

    #[error("failed to parse {path:?}")]
    ParseFailed { path: std::path::PathBuf },
}
