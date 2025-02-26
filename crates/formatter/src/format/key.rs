use std::fmt::Write;

use ast::AstNode;
use itertools::Itertools;

use crate::Format;

impl Format for ast::Keys {
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        let keys = self
            .keys()
            .map(|key| key.syntax().text().to_string())
            .collect_vec()
            .join(".");

        write!(f, "{}", keys)
    }
}

impl Format for ast::BareKey {
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.syntax().text())
    }
}

impl Format for ast::Key {
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::BareKey(it) => it.format(f),
            Self::BasicString(it) => it.format(f),
            Self::LiteralString(it) => it.format(f),
        }
    }
}
