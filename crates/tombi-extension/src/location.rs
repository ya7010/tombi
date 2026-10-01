/// A location in a document.
#[derive(Debug, Clone)]
pub struct Location {
    pub uri: tombi_uri::Uri,
    /// The span in the TOML document of `uri`.
    ///
    /// `None` opens the file at its start, for a file that is not parsed, such as a source file.
    pub span: Option<LocatedSpan>,
}

/// A span with the line index of its document, built while parsing it, to convert the span.
#[derive(Debug, Clone)]
pub struct LocatedSpan {
    pub span: tombi_text::Span,
    pub line_index: std::sync::Arc<tombi_text::LineIndex>,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.uri == other.uri
            && self.span.as_ref().map(|span| span.span) == other.span.as_ref().map(|span| span.span)
    }
}

impl Eq for Location {}

impl std::hash::Hash for Location {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uri.hash(state);
        self.span.as_ref().map(|span| span.span).hash(state);
    }
}
