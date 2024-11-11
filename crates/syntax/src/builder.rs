use rg_tree::Language;

use crate::TomlLanguage;

#[derive(Default)]
pub struct SyntaxTreeBuilder {
    errors: Vec<crate::SyntaxError>,
    inner: rg_tree::GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    pub fn finish(self) -> (rg_tree::GreenNode, Vec<crate::SyntaxError>) {
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

    pub fn error(&mut self, message: String, range: text::Range) {
        self.errors.push(crate::SyntaxError::new(message, range));
    }
}
