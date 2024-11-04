use syntax::T;

use crate::{
    grammar::{leading_comments, peek_leading_comments, tailing_comment, value::parse_value},
    parser::Parser,
};
use syntax::SyntaxKind::*;

pub fn parse_array(p: &mut Parser<'_>) {
    let m = p.start();

    leading_comments(p);

    assert!(p.at(T!['[']));

    p.eat(T!['[']);

    loop {
        let n = peek_leading_comments(p);
        if p.nth_at(n, EOF) || p.nth_at(n, T![']']) {
            break;
        }

        parse_value(p);
        while p.eat(WHITESPACE) || p.eat(NEWLINE) || p.eat(COMMENT) {}
        p.eat(T![,]);
        tailing_comment(p);
    }

    if !p.eat(T![']']) {
        p.error(crate::Error::ExpectedBracketEnd);
    }

    tailing_comment(p);

    m.complete(p, ARRAY);
}
