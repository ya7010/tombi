use syntax::{
    SyntaxKind::{self, *},
    T,
};

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
        p.error("expected '='");
    }
    parse_value(p);

    m.complete(p, KEY_VALUE);
}

pub fn parse_key(p: &mut Parser<'_>) {
    let m = p.start();

    if p.at_ts(KEY_FIRST) {
        p.bump_any();
    } else {
        p.error("expected key");
    }

    // FIXME: This should be a KEY token.
    m.complete(p, SyntaxKind::BARE_KEY);
}

pub fn parse_value(p: &mut Parser<'_>) {
    let m = p.start();
    if p.at_ts(VALUE_FIRST) {
        p.bump_any();
    } else {
        p.error("expected value");
    }

    // FIXME: This should be a VALUE token.
    m.complete(p, SyntaxKind::BOOLEAN);
}
