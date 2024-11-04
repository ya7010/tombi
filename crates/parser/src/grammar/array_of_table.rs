use syntax::T;

use crate::{
    grammar::{
        key::parse_keys, key_value::parse_key_value, leading_comments, peek_leading_comments,
        root::NEXT_SECTION, tailing_comment,
    },
    parser::Parser,
};
use syntax::SyntaxKind::*;

pub fn parse_array_of_table(p: &mut Parser<'_>) {
    let m = p.start();

    leading_comments(p);

    assert!(p.at(T!("[[")));

    p.eat(T!("[["));

    parse_keys(p);

    if !p.eat(T!("]]")) {
        p.error(crate::Error::ExpectedDoubleBracetEnd);
    }

    tailing_comment(p);

    loop {
        let n = peek_leading_comments(p);

        if p.nth_at_ts(n, NEXT_SECTION) {
            break;
        }
        parse_key_value(p);
    }

    m.complete(p, ARRAY_OF_TABLE);
}
