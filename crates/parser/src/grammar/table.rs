use syntax::T;

use crate::parser::Parser;

pub fn parse_table(p: &mut Parser<'_>) {
    while p.at(T!['[']) && p.nth(1) != T!['['] {
        parse_header(p);
    }
}

fn parse_header(p: &mut Parser<'_>) {
    assert!(p.at(T!['[']));
}
