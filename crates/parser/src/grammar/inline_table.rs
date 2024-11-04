use syntax::{SyntaxKind::*, T};

use crate::{
    grammar::{leading_comments, peek_leading_comments, tailing_comment},
    parser::Parser,
};

use super::key_value::parse_key_value;

pub fn parse_inline_table(p: &mut Parser<'_>) {
    let m = p.start();

    leading_comments(p);

    assert!(p.at(T!['{']));

    p.eat(T!['{']);

    loop {
        let n = peek_leading_comments(p);
        if p.nth_at(n, EOF) || p.nth_at(n, T!['}']) {
            break;
        }
        parse_key_value(p);
        p.eat(T![,]);
        tailing_comment(p);
    }

    if !p.eat(T!['}']) {
        p.error(crate::Error::ExpectedBraceEnd);
    }

    tailing_comment(p);

    m.complete(p, INLINE_TABLE);
}
