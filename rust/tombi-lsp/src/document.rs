use crate::converters::line_index::LineIndex;

#[derive(Debug)]
pub struct Document {
    source: String,
    line_index: LineIndex,
    document: Option<document::Table>,
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

    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    pub fn line_index(&self) -> &LineIndex {
        &self.line_index
    }

    pub fn document(&mut self) -> &document::Table {
        // TODO: Implement this
        // if self.document.is_none() {
        //     self.document = Some(document::Table::new(&self.ast));
        // }
        self.document.as_ref().unwrap()
    }
}
