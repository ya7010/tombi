use syntax::{SyntaxKind::*, T};

use crate::{
    grammar::{leading_comments, tailing_comment},
    marker::Marker,
    parser::Parser,
};

use super::key_value::parse_key_value;

pub fn parse_inline_table(p: &mut Parser<'_>, m: Marker) {
    assert!(p.at(T!['{']));

    p.eat(T!['{']);

    loop {
        let child_m = p.start();
        leading_comments(p);
        if p.at(EOF) || p.at(T!['}']) {
            child_m.abandon(p);
            break;
        }
        parse_key_value(p, child_m);
        p.eat(T![,]);
        tailing_comment(p);
    }

    if !p.eat(T!['}']) {
        p.error(crate::Error::ExpectedBraceEnd);
    }

    tailing_comment(p);

    m.complete(p, INLINE_TABLE);
}
