use crate::{
    grammar::{array_of_table::parse_array_of_table, table::parse_table},
    marker::Marker,
    parser::Parser,
    token_set::TokenSet,
};

use super::{key::KEY_FIRST, key_value::parse_key_value, leading_comments};
use syntax::{SyntaxKind::*, T};

const LINE_END: TokenSet = TokenSet::new(&[NEWLINE, COMMENT, EOF]);
pub const NEXT_SECTION: TokenSet = TokenSet::new(&[T!['['], T!("[["), EOF]);

pub fn parse_root(p: &mut Parser<'_>) {
    let m = p.start();

    loop {
        let m = p.start();
        leading_comments(p);
        if p.at(EOF) {
            m.abandon(p);
            break;
        }
        if p.at_ts(KEY_FIRST) {
            parse_key_value(p, m);
        } else if p.at(T!("[[")) {
            parse_array_of_table(p, m);
        } else if p.at(T!['[']) {
            parse_table(p, m);
        } else {
            parse_unknwon_line(p, m);
        }
    }
    m.complete(p, ROOT);
}

fn parse_unknwon_line(p: &mut Parser<'_>, m: Marker) {
    while !p.at_ts(LINE_END) {
        p.bump_any();
    }
    p.error(crate::Error::UnknownLine);
    m.complete(p, ERROR);
}
