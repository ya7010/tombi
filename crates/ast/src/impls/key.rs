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
