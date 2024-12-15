use crate::TryIntoDocumentTree;

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
    pub fn try_new(
        kind: KeyKind,
        value: String,
        range: text::Range,
    ) -> Result<Self, crate::support::string::ParseError> {
        let key = Self { kind, value, range };
        key.try_to_raw_string()?;

        Ok(key)
    }
    pub fn kind(&self) -> KeyKind {
        self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn to_raw_text(&self) -> String {
        // NOTE: Key has already been validated by `impl TryIntoDocumentTree<Key>`,
        //       so it's safe to unwrap.
        self.try_to_raw_string().unwrap()
    }

    fn try_to_raw_string(&self) -> Result<std::string::String, crate::support::string::ParseError> {
        match self.kind {
            KeyKind::BareKey => Ok(crate::support::string::from_bare_key(&self.value)),
            KeyKind::BasicString => crate::support::string::try_from_basic_string(&self.value),
            KeyKind::LiteralString => crate::support::string::try_from_literal_string(&self.value),
        }
    }

    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.try_to_raw_string() == other.try_to_raw_string()
    }
}

impl Eq for Key {}

impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.try_to_raw_string()
            .unwrap_or_else(|_| self.value.to_string())
            .hash(state);
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryIntoDocumentTree<Key> for ast::Key {
    fn try_into_document_tree(
        self,
        _toml_version: config::TomlVersion,
    ) -> Result<Key, Vec<crate::Error>> {
        let token = self.token().unwrap();

        Key::try_new(
            match self {
                ast::Key::BareKey(_) => KeyKind::BareKey,
                ast::Key::BasicString(_) => KeyKind::BasicString,
                ast::Key::LiteralString(_) => KeyKind::LiteralString,
            },
            token.text().to_string(),
            token.range(),
        )
        .map_err(|error| {
            vec![crate::Error::ParseStringError {
                error,
                range: token.range(),
            }]
        })
    }
}

impl From<Key> for String {
    fn from(value: Key) -> Self {
        value.try_to_raw_string().unwrap()
    }
}
