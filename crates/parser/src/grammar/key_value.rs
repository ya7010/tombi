use super::{key::parse_keys, tailing_comment, value::parse_value};
use crate::{marker::Marker, parser::Parser};
use syntax::{SyntaxKind::*, T};

pub fn parse_key_value(p: &mut Parser<'_>, m: Marker) {
    parse_keys(p);

    if !p.eat(T![=]) {
        p.error(crate::Error::ExpectedEquals);
    }

    let child_m = p.start();
    parse_value(p, child_m);

    tailing_comment(p);

    m.complete(p, KEY_VALUE);
}
