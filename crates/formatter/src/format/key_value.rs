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
            ast::Value::Boolean(it) => it.format(context),
            _ => unimplemented!("Value::format is not implemented for {:?}", self),
        }
    }
}
