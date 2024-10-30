use crate::converters::line_index::LineIndex;

#[derive(Debug, Clone)]
pub struct Document {
    pub source: String,
    pub line_index: LineIndex,
    pub document: Option<document::Table>,
}

impl Document {
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let line_index = LineIndex::new(&source);

        Self {
            source,
            line_index,
            document: None,
        }
    }
}
