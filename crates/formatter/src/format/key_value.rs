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
            _ => unimplemented!("Key::format is not implemented for {:?}", self),
        }
    }
}

impl Format for ast::BareKey {
    fn format<'a>(&self, _context: &'a crate::Context<'a>) -> String {
        self.to_string()
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
