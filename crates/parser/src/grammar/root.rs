use crate::parser::Parser;

use super::key_value::{parse_key_value, KEY_FIRST};
use syntax::SyntaxKind::*;

pub fn parse(p: &mut Parser<'_>) {
    let m = p.start();
    while !p.at(EOF) {
        if p.at_ts(KEY_FIRST) {
            parse_key_value(p);
        } else {
            p.error(crate::Error::UnknownToken);
            p.bump_any()
        }
    }
    m.complete(p, ROOT);
}
