#[derive(Debug, Clone)]
pub struct DocumentSource {
    pub source: String,
}

impl DocumentSource {
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        Self { source }
    }
}
