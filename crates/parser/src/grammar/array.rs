use syntax::T;

use crate::{
    grammar::{leading_comments, tailing_comment, value::parse_value},
    parser::Parser,
};
use syntax::SyntaxKind::*;

pub fn parse_array(p: &mut Parser<'_>) {
    let m = p.start();

    leading_comments(p);

    assert!(p.at(T!['[']));

    p.eat(T!['[']);

    loop {
        if p.at(EOF) || p.at(T![']']) {
            break;
        }

        parse_value(p);
        p.eat(T![,]);
        tailing_comment(p);
    }

    if !p.eat(T![']']) {
        p.error(crate::Error::ExpectedBracketEnd);
    }

    m.complete(p, ARRAY);
}
