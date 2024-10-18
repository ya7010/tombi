use super::{key::parse_key, value::parse_value};
use crate::parser::Parser;
use syntax::{SyntaxKind::*, T};

pub fn parse_key_value(p: &mut Parser<'_>) {
    let m = p.start();

    parse_key(p);

    if !p.eat(T![=]) {
        p.error(crate::Error::ExpectedEquals);
    }

    parse_value(p);

    m.complete(p, KEY_VALUE);
}
