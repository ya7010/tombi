use syntax::T;

use crate::{
    grammar::{tailing_comment, value::parse_value},
    marker::Marker,
    parser::Parser,
};
use syntax::SyntaxKind::*;

pub fn parse_array(p: &mut Parser<'_>, m: Marker) {
    assert!(p.at(T!['[']));

    p.eat(T!['[']);

    loop {
        let child_m = p.start();
        if p.at(EOF) || p.at(T![']']) {
            child_m.abandon(p);
            break;
        }

        parse_value(p, child_m);
        p.eat(T![,]);
        tailing_comment(p);
    }

    if !p.eat(T![']']) {
        p.error(crate::Error::ExpectedBracketEnd);
    }

    m.complete(p, ARRAY);
}
