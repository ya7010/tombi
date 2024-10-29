use syntax::T;

use crate::{
    grammar::{line_end, value::parse_value},
    parser::Parser,
};
use syntax::SyntaxKind::*;

pub fn parse_array(p: &mut Parser<'_>) {
    assert!(p.at(T!['[']));

    let m = p.start();
    p.eat(T!['[']);

    while !p.at(EOF) && !p.at(T![']']) {
        line_end(p);
        parse_value(p);
        line_end(p);
        p.eat(T![,]);
        line_end(p);
    }

    if !p.eat(T![']']) {
        p.error(crate::Error::ExpectedBracketEnd);
    }

    m.complete(p, ARRAY);
}
