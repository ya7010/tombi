#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyKind {
    BareKey,
    BasicString,
    LiteralString,
}

impl From<document_tree::KeyKind> for KeyKind {
    fn from(kind: document_tree::KeyKind) -> Self {
        match kind {
            document_tree::KeyKind::BareKey => Self::BareKey,
            document_tree::KeyKind::BasicString => Self::BasicString,
            document_tree::KeyKind::LiteralString => Self::LiteralString,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Key {
    kind: KeyKind,
    value: String,
}

impl Key {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn to_raw_text(&self) -> String {
        match self.kind {
            KeyKind::BareKey => self.value.to_string(),
            KeyKind::BasicString => self.value[1..self.value.len() - 1].replace(r#"\""#, "\""),
            KeyKind::LiteralString => self.value[1..self.value.len() - 1].replace(r#"\'"#, "'to'"),
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.to_raw_text() == other.to_raw_text()
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

impl From<document_tree::Key> for Key {
    fn from(node: document_tree::Key) -> Self {
        Self {
            kind: node.kind().into(),
            value: node.into(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use crate::test_serialize;
    use serde_json::json;

    test_serialize! {
        #[test]
        fn bare_key(r#"key = 1"#) -> Ok(json!({"key": 1}))
    }

    test_serialize! {
        #[test]
        fn basic_string_key(r#""key" = 1"#) -> Ok(json!({"key": 1}))
    }

    test_serialize! {
        #[test]
        fn literal_string_key(r#"'key' = 'value'"#) -> Ok(json!({"key": "value"}))
    }
}
