mod formatter;
mod level;

pub use formatter::Pretty;
pub use level::Level;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    level: level::Level,
    message: String,
    position: text_size::TextPosition,
    range: text_size::TextRange,
    source_file: Option<std::path::PathBuf>,
}

impl Diagnostic {
    pub fn new_warnig(message: String, source: &str, range: text_size::TextRange) -> Self {
        Self {
            level: level::Level::Warning,
            message,
            position: text_size::TextPosition::from_source(source, range.start()),
            range,
            source_file: None,
        }
    }

    pub fn new_error(message: String, source: &str, range: text_size::TextRange) -> Self {
        Self {
            level: level::Level::Error,
            message,
            position: text_size::TextPosition::from_source(source, range.start()),
            range,
            source_file: None,
        }
    }

    pub fn with_source_file(mut self, source_file: impl Into<std::path::PathBuf>) -> Self {
        self.source_file = Some(source_file.into());
        self
    }

    pub fn level(&self) -> level::Level {
        self.level
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn position(&self) -> text_size::TextPosition {
        self.position
    }

    pub fn range(&self) -> text_size::TextRange {
        self.range
    }

    pub fn source_file(&self) -> Option<&std::path::Path> {
        self.source_file.as_deref()
    }
}

pub trait Print<Target> {
    /// Formats the object using the given formatter.
    fn print(&self, target: &Target);
}
