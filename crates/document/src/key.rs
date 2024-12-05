#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    value: String,
    pub(crate) range: text::Range,
}

impl Key {
    pub(crate) fn new(text: &str, range: text::Range) -> Self {
        Self {
            value: text.to_string(),
            range,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<ast::Key> for Key {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::Key) -> Result<Self, Self::Error> {
        let token = match node {
            ast::Key::BareKey(bare_key) => bare_key.token().unwrap(),
            ast::Key::BasicString(basic_string) => basic_string.token().unwrap(),
            ast::Key::LiteralString(literal_string) => literal_string.token().unwrap(),
        };
        Ok(Key::new(token.text(), token.text_range()))
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
