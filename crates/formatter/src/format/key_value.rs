use ast::AstNode;

use crate::children_kinds;

use super::Format;

impl Format for ast::KeyValue {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        println!("{:?}", children_kinds::<syntax::SyntaxKind>(self.syntax()));
        println!(
            "key: {:?}, eq: {:?}, value: {:?}",
            self.key(),
            self.eq_token(),
            self.value()
        );
        self.key().unwrap().format(context) + " = " + &self.value().unwrap().format(context)
    }
}

impl Format for ast::Key {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        match self {
            Self::BareKey(it) => it.format(context),
            Self::QuotedKey(it) => it.format(context),
            Self::DottedKeys(it) => it.format(context),
        }
    }
}

impl Format for ast::BareKey {
    fn format<'a>(&self, _context: &'a crate::Context<'a>) -> String {
        self.to_string()
    }
}

impl Format for ast::QuotedKey {
    fn format<'a>(&self, _context: &'a crate::Context<'a>) -> String {
        self.to_string()
    }
}

impl Format for ast::SingleKey {
    fn format<'a>(&self, _context: &'a crate::Context<'a>) -> String {
        match self {
            ast::SingleKey::BareKey(it) => it.format(_context),
            ast::SingleKey::QuotedKey(it) => it.format(_context),
        }
    }
}

impl Format for ast::DottedKeys {
    fn format<'a>(&self, _context: &'a crate::Context<'a>) -> String {
        dbg!(self.single_keys());
        self.single_keys()
            .into_iter()
            .map(|it| it.format(_context))
            .collect::<Vec<_>>()
            .join(".")
    }
}

impl Format for ast::Value {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        match self {
            ast::Value::Array(it) => it.format(context),
            ast::Value::BasicString(it) => it.format(context),
            ast::Value::Boolean(it) => it.format(context),
            ast::Value::Float(it) => it.format(context),
            ast::Value::InlineTable(it) => it.format(context),
            ast::Value::IntegerBin(it) => it.format(context),
            ast::Value::IntegerDec(it) => it.format(context),
            ast::Value::IntegerHex(it) => it.format(context),
            ast::Value::IntegerOct(it) => it.format(context),
            ast::Value::LiteralString(it) => it.format(context),
            ast::Value::LocalDate(it) => it.format(context),
            ast::Value::LocalDateTime(it) => it.format(context),
            ast::Value::LocalTime(it) => it.format(context),
            ast::Value::MultiLineBasicString(it) => it.format(context),
            ast::Value::MultiLineLiteralString(it) => it.format(context),
            ast::Value::OffsetDateTime(it) => it.format(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use syntax::SyntaxError;

    #[rstest]
    #[case(r#"key = "value""#)]
    #[case(r#"key    = "value""#)]
    fn bare_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        let ast = ast::Root::cast(p.syntax_node()).unwrap();
        assert_eq!(ast.format_default(), "key = \"value\"");
        assert_eq!(p.errors().len(), 0);
    }

    #[rstest]
    #[case("key = # INVALID")]
    fn invalid_bare_key_value(#[case] source: &str) {
        let p = parser::parse(source);
        assert_ne!(p.errors().len(), 0);
        assert_eq!(
            p.errors(),
            vec![SyntaxError::new(parser::Error::ExpectedValue, 4..6)]
        );
    }
}
