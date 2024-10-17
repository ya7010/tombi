use syntax::{
    SyntaxKind::{self, *},
    T,
};

use crate::{parser::Parser, token_set::TokenSet};

use super::inline_table::parse_inline_table;

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
]);

pub(crate) const VALUE_FIRST: TokenSet = TokenSet::new(&[
    SyntaxKind::BASIC_STRING,
    SyntaxKind::MULTI_LINE_BASIC_STRING,
    SyntaxKind::LITERAL_STRING,
    SyntaxKind::MULTI_LINE_LITERAL_STRING,
    SyntaxKind::INTEGER_BIN,
    SyntaxKind::INTEGER_OCT,
    SyntaxKind::INTEGER_DEC,
    SyntaxKind::INTEGER_HEX,
    SyntaxKind::FLOAT,
    SyntaxKind::BOOLEAN,
    SyntaxKind::OFFSET_DATE_TIME,
    SyntaxKind::LOCAL_DATE_TIME,
    SyntaxKind::LOCAL_DATE,
    SyntaxKind::LOCAL_TIME,
    SyntaxKind::BRACE_START,
    SyntaxKind::BRACKET_START,
]);

pub fn parse_key_value(p: &mut Parser<'_>) {
    let m = p.start();

    parse_key(p);

    if !p.eat(T![=]) {
        p.error(crate::Error::ExpectedEquals);
    }

    parse_value(p);

    p.eat(COMMENT);

    m.complete(p, KEY_VALUE);
}

pub fn parse_key(p: &mut Parser<'_>) {
    let m = p.start();
    if let Some(kind) = eat_key(p) {
        m.complete(p, kind);
    } else {
        p.error(crate::Error::ExpectedKey);
        m.complete(p, INVALID_TOKENS);
    };
}

fn eat_key(p: &mut Parser<'_>) -> Option<SyntaxKind> {
    if p.nth_at(1, T![.]) {
        // Dotted keys Mode
        loop {
            let m = p.start();
            if let Some(kind) = eat_single_key(p) {
                m.complete(p, kind);
            } else {
                p.error(crate::Error::ExpectedKey);
                m.complete(p, INVALID_TOKENS);
                return None;
            }
            if !p.eat(T![.]) {
                break;
            }
        }
        Some(DOTTED_KEYS)
    } else {
        eat_single_key(p)
    }
}

pub fn eat_single_key(p: &mut Parser<'_>) -> Option<SyntaxKind> {
    let kind = p.current();
    match kind {
        BARE_KEY | BASIC_STRING | LITERAL_STRING => {
            p.bump_any();
            Some(kind)
        }
        _ => None,
    }
}

pub fn parse_value(p: &mut Parser<'_>) {
    match p.current() {
        BASIC_STRING
        | MULTI_LINE_BASIC_STRING
        | LITERAL_STRING
        | MULTI_LINE_LITERAL_STRING
        | INTEGER_DEC
        | INTEGER_BIN
        | INTEGER_OCT
        | INTEGER_HEX
        | FLOAT
        | BOOLEAN
        | OFFSET_DATE_TIME
        | LOCAL_DATE_TIME
        | LOCAL_DATE
        | LOCAL_TIME => {
            let m = p.start();
            let kind = p.current();
            p.bump(kind);
            m.complete(p, kind);
        }
        BRACE_START => parse_inline_table(p),
        _ => {
            let m = p.start();
            while !p.at_ts(TokenSet::new(&[NEWLINE, COMMENT, EOF])) {
                p.bump_any();
            }
            p.error(crate::Error::ExpectedValue);
            m.complete(p, INVALID_TOKENS);
        }
    }
}
