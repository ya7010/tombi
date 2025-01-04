use syntax::{
    SyntaxKind::{self, *},
    T,
};

use crate::{parser::Parser, token_set::TokenSet};

use super::Parse;
use crate::ErrorKind::*;

pub(crate) const KEY_FIRST: TokenSet = TokenSet::new(&[
    // name = "Tom"
    SyntaxKind::BARE_KEY,
    // "127.0.0.1" = "value"
    SyntaxKind::BASIC_STRING,
    // 'key2' = "value"
    SyntaxKind::LITERAL_STRING,
    // 1234 = "value"
    SyntaxKind::INTEGER_DEC,
    // 3.14159 = "pi"
    SyntaxKind::FLOAT,
    // true = "value"
    SyntaxKind::BOOLEAN,
]);

impl Parse for ast::Keys {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();
        if eat_keys(p) {
            m.complete(p, KEYS);
        } else {
            m.complete(p, INVALID_TOKEN);
        };
    }
}

fn eat_keys(p: &mut Parser<'_>) -> bool {
    if p.nth_at(1, T![.]) {
        let mut is_error = false;
        // Dotted keys Mode
        loop {
            is_error |= !eat_key(p);
            if !p.eat(T![.]) {
                break;
            }
        }
        !is_error
    } else {
        eat_key(p)
    }
}

pub fn eat_key(p: &mut Parser<'_>) -> bool {
    let kind = p.current();
    match kind {
        BARE_KEY | BASIC_STRING | LITERAL_STRING => {
            let m = p.start();
            p.bump(kind);
            m.complete(p, kind);
            true
        }
        INTEGER_DEC | BOOLEAN => {
            let m = p.start();
            p.bump_remap(BARE_KEY);
            m.complete(p, BARE_KEY);
            true
        }
        FLOAT => {
            p.bump_float_key();
            true
        }
        _ => {
            let m = p.start();
            p.error(crate::Error::new(ExpectedKey, p.current_range()));
            // p.bump_remap(INVALID_TOKEN);
            m.complete(p, INVALID_TOKEN);
            return false;
        }
    }
}
