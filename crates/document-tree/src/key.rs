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

    pub fn to_raw_string(&self) -> std::string::String {
        match self.kind {
            KeyKind::BareKey => crate::support::string::from_bare_key(&self.value),
            KeyKind::BasicString => crate::support::string::from_basic_string(&self.value),
            KeyKind::LiteralString => crate::support::string::from_literal_string(&self.value),
        }
    }

    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.to_raw_string() == other.to_raw_string()
    }
}

impl Eq for Key {}

impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_raw_string().hash(state);
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<ast::Key> for Key {
    fn from(node: ast::Key) -> Self {
        let token = node.token().unwrap();
        Self {
            kind: match node {
                ast::Key::BareKey(_) => KeyKind::BareKey,
                ast::Key::BasicString(_) => KeyKind::BasicString,
                ast::Key::LiteralString(_) => KeyKind::LiteralString,
            },
            value: token.text().to_string(),
            range: token.range(),
        }
    }
}

impl From<Key> for String {
    fn from(key: Key) -> Self {
        key.to_raw_string()
    }
}
