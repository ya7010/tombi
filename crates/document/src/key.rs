#[derive(Debug, Clone)]
pub struct Key {
    value: String,
    range: text::Range,
}

impl Key {
    pub(crate) fn new(text: impl ToString, range: text::Range) -> Self {
        Self {
            value: text.to_string(),
            range,
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
        Self::new(node.raw_text(), node.token().unwrap().text_range())
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
