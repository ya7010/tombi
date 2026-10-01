/// A diagnostic reported by Tombi, with its range resolved for the bindings.
///
/// The columns count grapheme clusters, as an editor shows them.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
#[cfg_attr(feature = "wasm", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "python", pyo3::pyclass(skip_from_py_object))]
pub struct Diagnostic {
    level: tombi_diagnostic::Level,
    code: String,
    message: String,
    range: tombi_text::Range,
    source_file: Option<std::path::PathBuf>,
}

impl Diagnostic {
    /// Converts the diagnostics of the source indexed by `line_index`,
    /// which is the one built while parsing the source.
    pub(crate) fn from_diagnostics(
        diagnostics: Vec<tombi_diagnostic::Diagnostic>,
        line_index: &tombi_text::LineIndex,
    ) -> Vec<Self> {
        diagnostics
            .iter()
            .map(|diagnostic| {
                let located =
                    diagnostic.located(line_index, tombi_text::EncodingKind::GraphemeCluster);
                Self {
                    level: diagnostic.level(),
                    code: diagnostic.code().to_owned(),
                    message: diagnostic.message().to_owned(),
                    range: located.range(),
                    source_file: diagnostic.source_file().map(ToOwned::to_owned),
                }
            })
            .collect()
    }

    #[inline]
    pub fn level(&self) -> tombi_diagnostic::Level {
        self.level
    }

    #[inline]
    pub fn is_error(&self) -> bool {
        self.level == tombi_diagnostic::Level::ERROR
    }

    #[inline]
    pub fn code(&self) -> &str {
        &self.code
    }

    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[inline]
    pub fn range(&self) -> tombi_text::Range {
        self.range
    }

    #[inline]
    pub fn source_file(&self) -> Option<&std::path::Path> {
        self.source_file.as_deref()
    }
}

/// A zero-based position in a TOML document, exposed to Python with
/// dot-chain access (`diagnostic.range.start.line`). A Python-only
/// counterpart to `tombi_text::Position`, kept in this crate rather than
/// adding a `python` feature to `tombi-text` itself.
#[cfg(feature = "python")]
#[derive(Debug, Clone, Copy)]
#[pyo3::pyclass(get_all, skip_from_py_object)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl Position {
    fn __repr__(&self) -> String {
        format!("Position(line={}, column={})", self.line, self.column)
    }
}

#[cfg(feature = "python")]
impl From<tombi_text::Position> for Position {
    fn from(position: tombi_text::Position) -> Self {
        Self {
            line: position.line,
            column: position.column,
        }
    }
}

/// A range in a TOML document, exposed to Python with dot-chain access
/// (`diagnostic.range.start`/`.end`). See [`Position`] for why this isn't
/// `tombi_text::Range` directly.
#[cfg(feature = "python")]
#[derive(Debug, Clone, Copy)]
#[pyo3::pyclass(get_all, skip_from_py_object)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl Range {
    fn __repr__(&self) -> String {
        format!("Range(start={:?}, end={:?})", self.start, self.end)
    }
}

#[cfg(feature = "python")]
impl From<tombi_text::Range> for Range {
    fn from(range: tombi_text::Range) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl Diagnostic {
    #[getter(level)]
    fn py_level(&self) -> &str {
        match self.level {
            tombi_diagnostic::Level::WARNING => "warning",
            tombi_diagnostic::Level::ERROR => "error",
        }
    }

    #[getter(code)]
    fn py_code(&self) -> &str {
        self.code()
    }

    #[getter(message)]
    fn py_message(&self) -> &str {
        self.message()
    }

    #[getter(range)]
    fn py_range(&self) -> Range {
        self.range.into()
    }

    #[getter(source_file)]
    fn py_source_file(&self) -> Option<String> {
        self.source_file()
            .map(|source_file| source_file.to_string_lossy().into_owned())
    }

    fn __repr__(&self) -> String {
        format!(
            "Diagnostic(level={:?}, code={:?}, message={:?})",
            self.py_level(),
            self.code(),
            self.message()
        )
    }
}
