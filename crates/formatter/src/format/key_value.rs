use ast::AstNode;

use crate::Format;
use std::fmt::Write;

impl Format for ast::KeyValue {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        self.leading_comments()
            .iter()
            .map(|comment| write!(f, "{}\n", comment))
            .collect::<Result<(), std::fmt::Error>>()?;
        self.keys().unwrap().fmt(f)?;
        write!(f, " = ")?;
        self.value().unwrap().fmt(f)?;

        Ok(())
    }
}

impl Format for ast::Keys {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        let keys = self
            .keys()
            .map(|key| key.syntax().text().to_string())
            .collect::<Vec<_>>()
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

impl Format for ast::Value {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Array(it) => it.fmt(f),
            Self::BasicString(it) => it.fmt(f),
            Self::Boolean(it) => it.fmt(f),
            Self::Float(it) => it.fmt(f),
            Self::InlineTable(it) => it.fmt(f),
            Self::IntegerBin(it) => it.fmt(f),
            Self::IntegerDec(it) => it.fmt(f),
            Self::IntegerHex(it) => it.fmt(f),
            Self::IntegerOct(it) => it.fmt(f),
            Self::LiteralString(it) => it.fmt(f),
            Self::LocalDate(it) => it.fmt(f),
            Self::LocalDateTime(it) => it.fmt(f),
            Self::LocalTime(it) => it.fmt(f),
            Self::MultiLineBasicString(it) => it.fmt(f),
            Self::MultiLineLiteralString(it) => it.fmt(f),
            Self::OffsetDateTime(it) => it.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::AstNode;

    use rstest::rstest;
    use syntax::SyntaxError;

    use crate::Format;

    #[rstest]
    #[case(r#"key = "value""#)]
    #[case(r#"key    = "value""#)]
    fn bare_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, "key = \"value\"");
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case("key = # INVALID")]
    fn invalid_bare_key_value(#[case] source: &str) {
        let p = parser::parse(source);

        assert_eq!(
            p.errors(),
            vec![SyntaxError::new(parser::Error::ExpectedValue, 4..15)]
        );
    }

    #[rstest]
    #[case(r#"key1.key2.key3 = "value""#)]
    #[case(r#"site."google.com" = true"#)]
    fn dotted_keys_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }

    #[rstest]
    #[case(
        r#"
# leading comment1
# leading comment2
key = "value"  # tailing comment
    "#.trim()
    )]
    fn key_value_with_comment(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();

        let mut formatted_text = String::new();
        ast.fmt(&mut crate::Formatter::new(&mut formatted_text))
            .unwrap();

        assert_eq!(formatted_text, source);
        assert_eq!(p.errors(), []);
    }
}
