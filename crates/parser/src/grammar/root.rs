use crate::{parser::Parser, token_set::TokenSet};

use super::key_value::{parse_key_value, KEY_FIRST};
use syntax::SyntaxKind::*;

const LINE_END: TokenSet = TokenSet::new(&[NEWLINE, COMMENT, EOF]);

pub fn parse_root(p: &mut Parser<'_>) {
    let m = p.start();

    while p.eat(NEWLINE) || p.eat(COMMENT) {}
    while !p.at(EOF) {
        if p.at_ts(KEY_FIRST) {
            parse_key_value(p);
        } else {
            let m = p.start();
            while !p.at_ts(LINE_END) {
                p.bump_any();
            }
            p.error(crate::Error::UnknownToken);
            m.complete(p, ERROR);
        }
        while p.eat(NEWLINE) || p.eat(COMMENT) {}
        dbg!(p.current());
    }
    m.complete(p, ROOT);
}
