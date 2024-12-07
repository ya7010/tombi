impl crate::Key {
    pub fn token(&self) -> Option<syntax::SyntaxToken> {
        match self {
            Self::BareKey(key) => key.token(),
            Self::BasicString(key) => key.token(),
            Self::LiteralString(key) => key.token(),
        }
    }
}
