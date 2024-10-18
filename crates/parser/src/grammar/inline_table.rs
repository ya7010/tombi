use syntax::{SyntaxKind::*, T};

use crate::{grammar::line_end, parser::Parser};

use super::key_value::parse_key_value;

pub fn parse_inline_table(p: &mut Parser<'_>) {
    assert!(p.at(T!['{']));

    let m = p.start();
    p.eat(T!['{']);

    while !p.at(EOF) && !p.at(T!['}']) {
        parse_key_value(p);
        line_end(p);
        p.eat(T![,]);
        line_end(p);
    }

    if !p.eat(T!['}']) {
        p.error(crate::Error::ExpectedBraceEnd);
    }

    m.complete(p, INLINE_TABLE);
}
