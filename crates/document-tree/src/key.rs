use text::raw_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyKind {
    BareKey,
    BasicString,
    LiteralString,
}

#[derive(Debug, Clone)]
pub struct Key {
    kind: KeyKind,
    value: String,
    range: text::Range,
}

impl Key {
    pub fn kind(&self) -> KeyKind {
        self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn raw_string(&self) -> std::string::String {
        match self.kind {
            KeyKind::BareKey => raw_string::from_bare_key(&self.value),
            KeyKind::BasicString => raw_string::from_basic_string(&self.value),
            KeyKind::LiteralString => raw_string::from_literal_string(&self.value),
        }
    }

    pub fn range(&self) -> text::Range {
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
        Self {
            kind: match node {
                ast::Key::BareKey(_) => KeyKind::BareKey,
                ast::Key::BasicString(_) => KeyKind::BasicString,
                ast::Key::LiteralString(_) => KeyKind::LiteralString,
            },
            value: node.raw_text(),
            range: node.token().unwrap().range(),
        }
    }
}

impl From<Key> for String {
    fn from(key: Key) -> Self {
        key.raw_string()
    }
}
