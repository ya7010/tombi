use syntax::{
    SyntaxKind::{self, *},
    T,
};
use tracing::Instrument;

use crate::{parser::Parser, token_set::TokenSet};

pub(crate) const KEY_FIRST: TokenSet = TokenSet::new(&[
    // name = "Tom"
    SyntaxKind::BARE_KEY,
    // "127.0.0.1" = "value"
    SyntaxKind::BASIC_STRING,
    // 'key2' = "value"
    SyntaxKind::LITERAL_STRING,
    // 1234 = "value"
    SyntaxKind::INTEGER_BIN,
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
    if p.eat(NEWLINE) {
        return;
    }

    let m = p.start();

    parse_key(p);

    if !p.eat(T![=]) {
        p.error(crate::Error::ExpectedEquals);
    }
    parse_value(p);

    if p.at(COMMENT) {
        p.bump(COMMENT);
    }

    m.complete(p, KEY_VALUE);
}

pub fn parse_key(p: &mut Parser<'_>) {
    let m = p.start();

    dbg!(p.current());
    match p.current() {
        BARE_KEY | BASIC_STRING | LITERAL_STRING => {
            let kind = p.current();
            p.bump(kind);
            m.complete(p, kind);
        }
        _ => {
            p.error(crate::Error::ExpectedKey);
        }
    };
}

pub fn parse_value(p: &mut Parser<'_>) {
    let m = p.start();
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
            let kind = p.current();
            p.bump(kind);
            m.complete(p, kind);
        }
        _ => {
            while !p.at_ts(TokenSet::new(&[NEWLINE, COMMENT, EOF])) {
                p.bump_any();
            }
            p.error(crate::Error::ExpectedValue);
            m.complete(p, INVALID_TOKENS);
        }
    }
}
