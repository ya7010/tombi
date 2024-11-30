use super::Table;

#[derive(Debug)]
pub struct Root(Table);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Key {
    range: text::Range,
}

impl Key {
    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl From<ast::Key> for Key {
    fn from(node: ast::Key) -> Self {
        Self {
            range: match node {
                ast::Key::BareKey(key) => key.range(),
                ast::Key::BasicString(key) => key.range(),
                ast::Key::LiteralString(key) => key.range(),
            },
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
