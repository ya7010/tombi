use syntax::{SyntaxKind::*, T};

use crate::parser::Parser;

use super::key_value::parse_key_value;

pub fn parse_inline_table(p: &mut Parser<'_>) {
    let m = p.start();

    if !p.eat(T!['{']) {
        p.error(crate::Error::ExpectedEquals);
    }

    while !p.eat(NEWLINE) {
        parse_key_value(p);
        while p.eat(NEWLINE) || p.eat(COMMENT) {}
        p.eat(T![,]);
        while p.eat(NEWLINE) || p.eat(COMMENT) {}
        if p.eat(T!['}']) {
            break;
        }

        while p.eat(NEWLINE) || p.eat(COMMENT) {}
        if p.at(EOF) {
            p.error(crate::Error::ExpectedBracketEnd);
            break;
        }
    }

    if p.at(COMMENT) {
        p.bump(COMMENT);
    }

    m.complete(p, INLINE_TABLE);
}
