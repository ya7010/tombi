use std::path::PathBuf;

use diagnostics::{
    printer::{Pretty, Simple},
    Level, Print,
};
use nu_ansi_term::Style;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NotFormatted(#[from] NotFormattedError),
    #[error("{0:?} file not found")]
    FileNotFound(PathBuf),
    #[error("{0:?} is invalid glob pattern")]
    GlobPatternInvalid(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(thiserror::Error, Debug)]
pub struct NotFormattedError {
    source_path: Option<PathBuf>,
}

impl NotFormattedError {
    pub fn from_source(source_path: impl Into<PathBuf>) -> Self {
        Self {
            source_path: Some(source_path.into()),
        }
    }

    pub fn from_input() -> Self {
        Self { source_path: None }
    }

    pub fn into_error(self) -> Error {
        Error::NotFormatted(self)
    }
}

impl std::fmt::Display for NotFormattedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.source_path {
            Some(path) => write!(f, "{:?} is not formatted", path),
            None => write!(f, "Input is not formatted"),
        }
    }
}

impl Print<Pretty> for Error {
    fn print(&self, _printer: Pretty) {
        self.print(Simple);
    }
}

impl Print<Simple> for Error {
    fn print(&self, printer: Simple) {
        Level::Error.print(printer);
        println!(": {}", Style::new().bold().paint(self.to_string()));
    }
}
