#[derive(Debug, Clone)]
pub struct Document {
    pub source: String,
    pub document: Option<document::Table>,
}

impl Document {
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        Self {
            source,
            document: None,
        }
    }
}
