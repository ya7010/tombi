use super::{leading_comments, tailing_comment, Grammer};
use crate::parser::Parser;
use syntax::{SyntaxKind::*, T};

impl Grammer for ast::KeyValue {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        ast::Keys::parse(p);

        if !p.eat(T![=]) {
            p.error(crate::Error::ExpectedEquals);
        }

        ast::Value::parse(p);

        tailing_comment(p);

        m.complete(p, KEY_VALUE);
    }
}
