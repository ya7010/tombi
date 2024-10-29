mod level;
pub mod printer;

pub use level::Level;
pub use printer::Print;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    level: level::Level,
    message: String,
    position: text::Position,
    range: text::TextRange,
    source_file: Option<std::path::PathBuf>,
}

impl Diagnostic {
    pub fn new_warning(message: impl Into<String>, source: &str, range: text::TextRange) -> Self {
        Self {
            level: level::Level::Warning,
            message: message.into(),
            position: text::Position::from_source(source, range.start()),
            range,
            source_file: None,
        }
    }

    pub fn new_error(message: impl Into<String>, source: &str, range: text::TextRange) -> Self {
        Self {
            level: level::Level::Error,
            message: message.into(),
            position: text::Position::from_source(source, range.start()),
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

    pub fn position(&self) -> text::Position {
        self.position
    }

    pub fn range(&self) -> text::TextRange {
        self.range
    }

    pub fn source_file(&self) -> Option<&std::path::Path> {
        self.source_file.as_deref()
    }
}

pub trait OkOrPrint<T, E, P> {
    fn ok_or_print(self, printer: P) -> Option<T>;
}

impl<T, E, P> OkOrPrint<T, E, P> for Result<T, Vec<E>>
where
    E: Print<P>,
    P: Copy,
{
    fn ok_or_print(self, printer: P) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(diagnostics) => {
                for diagnostic in &diagnostics {
                    Print::<P>::print(diagnostic, printer);
                }
                None
            }
        }
    }
}
