use rowan::Language;

use crate::TomlLanguage;

#[derive(Default)]
pub struct SyntaxTreeBuilder {
    errors: Vec<crate::SyntaxError>,
    inner: rowan::GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    pub fn finish(self) -> (rowan::GreenNode, Vec<crate::SyntaxError>) {
        let green = self.inner.finish();
        (green, self.errors)
    }

    pub fn token(&mut self, kind: crate::SyntaxKind, text: &str) {
        let kind = TomlLanguage::kind_to_raw(kind);
        self.inner.token(kind, text);
    }

    pub fn start_node(&mut self, kind: crate::SyntaxKind) {
        let kind = TomlLanguage::kind_to_raw(kind);
        self.inner.start_node(kind);
    }

    pub fn finish_node(&mut self) {
        self.inner.finish_node();
    }

    pub fn error(&mut self, error: String, text_pos: rowan::TextSize) {
        self.errors.push(crate::SyntaxError::new(error, text_pos));
    }
}
