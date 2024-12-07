use crate::AstChildren;

impl crate::Key {
    pub fn token(&self) -> Option<syntax::SyntaxToken> {
        match self {
            Self::BareKey(key) => key.token(),
            Self::BasicString(key) => key.token(),
            Self::LiteralString(key) => key.token(),
        }
    }

    pub fn raw_text(&self) -> String {
        match self {
            Self::BareKey(key) => key.token().unwrap().text().to_string(),
            Self::BasicString(key) => key.token().unwrap().text()
                [1..key.token().unwrap().text().len() - 1]
                .replace(r#"\""#, "\"")
                .to_string(),
            Self::LiteralString(key) => key.token().unwrap().text()
                [1..key.token().unwrap().text().len() - 1]
                .replace(r#"\'"#, "'")
                .to_string(),
        }
    }
}

impl AstChildren<crate::Key> {
    pub fn starts_with(&self, other: &AstChildren<crate::Key>) -> bool {
        self.clone()
            .into_iter()
            .zip(other.clone().into_iter())
            .all(|(a, b)| a.raw_text() == b.raw_text())
    }

    #[inline]
    pub fn into_vec(self) -> Vec<crate::Key> {
        self.collect()
    }

    pub fn rev(self) -> impl Iterator<Item = crate::Key> {
        self.into_vec().into_iter().rev()
    }
}
