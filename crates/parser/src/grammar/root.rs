use crate::{parser::Parser, token_set::TokenSet};

use super::{
    key_value::{parse_key_value, KEY_FIRST},
    line_end,
};
use syntax::SyntaxKind::*;

const LINE_END: TokenSet = TokenSet::new(&[NEWLINE, COMMENT, EOF]);

pub fn parse_root(p: &mut Parser<'_>) {
    let m = p.start();

    line_end(p);
    while !p.at(EOF) {
        if p.at_ts(KEY_FIRST) {
            parse_key_value(p);
        } else {
            parse_unknwon_line(p);
        }
        line_end(p);
        dbg!(p.current());
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
