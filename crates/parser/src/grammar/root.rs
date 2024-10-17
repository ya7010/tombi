use crate::{
    grammar::{array_of_table::parse_array_of_table, table::parse_table},
    parser::Parser,
    token_set::TokenSet,
};

use super::{
    key_value::{parse_key_value, KEY_FIRST},
    line_end,
};
use syntax::{SyntaxKind::*, T};

const LINE_END: TokenSet = TokenSet::new(&[NEWLINE, COMMENT, EOF]);
pub const SECTION_END: TokenSet = TokenSet::new(&[T!['['], T!("[["), EOF]);

pub fn parse_root(p: &mut Parser<'_>) {
    let m = p.start();

    line_end(p);
    while !p.at(EOF) {
        if p.at_ts(KEY_FIRST) {
            parse_key_value(p);
        } else if p.at(T!("[[")) {
            parse_array_of_table(p);
        } else if p.at(T!['[']) {
            parse_table(p);
        } else {
            parse_unknwon_line(p);
        }
        line_end(p);
    }
    m.complete(p, ROOT);
}

fn parse_unknwon_line(p: &mut Parser<'_>) {
    let m = p.start();
    while !p.at_ts(LINE_END) {
        p.bump_any();
    }
    p.error(crate::Error::UnknownLine);
    m.complete(p, ERROR);
}
