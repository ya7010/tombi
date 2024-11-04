use super::{key::parse_keys, leading_comments, tailing_comment, value::parse_value};
use crate::parser::Parser;
use syntax::{SyntaxKind::*, T};

pub fn parse_key_value(p: &mut Parser<'_>) {
    let m = p.start();

    leading_comments(p);

    parse_keys(p);

    if !p.eat(T![=]) {
        p.error(crate::Error::ExpectedEquals);
    }

    parse_value(p);

    tailing_comment(p);

    m.complete(p, KEY_VALUE);
}
