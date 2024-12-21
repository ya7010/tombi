use ast::AstNode;
use itertools::Itertools;

use crate::Format;
use std::fmt::Write;

impl Format for ast::Keys {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        let keys = self
            .keys()
            .map(|key| key.syntax().text().to_string())
            .collect_vec()
            .join(".");

        write!(f, "{}", keys)
    }
}

impl Format for ast::BareKey {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.syntax().text())
    }
}

impl Format for ast::Key {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::BareKey(it) => it.fmt(f),
            Self::BasicString(it) => it.fmt(f),
            Self::LiteralString(it) => it.fmt(f),
        }
    }
}
