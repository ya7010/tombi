#[derive(Debug, Clone)]
pub struct Key {
    value: String,
    range: text::Range,
}

impl Key {
    pub(crate) fn new(node: &ast::Key) -> Self {
        Self {
            value: node.raw_text(),
            range: node.token().unwrap().text_range(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn range(&self) -> text::Range {
        self.range
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Key {}

impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<ast::Key> for Key {
    fn from(node: ast::Key) -> Self {
        Self::new(&node)
    }
}
