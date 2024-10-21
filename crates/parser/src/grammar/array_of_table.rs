use syntax::T;

use crate::{
    grammar::{key::parse_keys, key_value::parse_key_value, line_end, root::SECTION_END},
    parser::Parser,
};
use syntax::SyntaxKind::*;

pub fn parse_array_of_table(p: &mut Parser<'_>) {
    assert!(p.at(T!("[[")));

    let m = p.start();
    p.eat(T!("[["));

    parse_keys(p);

    if !p.eat(T!("]]")) {
        p.error(crate::Error::ExpectedDoubleBracetEnd);
    }

    line_end(p);

    while !p.at_ts(SECTION_END) {
        parse_key_value(p);
        line_end(p);
    }

    m.complete(p, ARRAY_OF_TABLE);
}
