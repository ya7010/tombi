use syntax::T;

use crate::{
    grammar::{
        key::parse_keys, key_value::parse_key_value, leading_comments, root::NEXT_SECTION,
        tailing_comment,
    },
    marker::Marker,
    parser::Parser,
};
use syntax::SyntaxKind::*;

pub fn parse_table(p: &mut Parser<'_>, m: Marker) {
    assert!(p.at(T!['[']));

    p.eat(T!['[']);

    parse_keys(p);

    if !p.eat(T![']']) {
        p.error(crate::Error::ExpectedBraceEnd);
    }

    tailing_comment(p);

    loop {
        let child_m = p.start();
        leading_comments(p);

        if p.at_ts(NEXT_SECTION) {
            child_m.abandon(p);
            break;
        }
        parse_key_value(p, child_m);
    }

    m.complete(p, TABLE);
}
