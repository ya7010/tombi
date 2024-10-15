use syntax::{
    SyntaxKind::{self, *},
    T,
};

use crate::{marker::CompletedMarker, parser::Parser, token_set::TokenSet};

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
    if !p.at_ts(KEY_FIRST) || !parse_key(p).is_some() {
        p.error("expected key");
    }

    if p.eat(T![=]) {
        parse_value(p);
    } else {
        p.error("expected '='");
    }

    parse_key(p);

    m.complete(p, KEY_VALUE);
}

pub fn parse_key(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.at_ts(KEY_FIRST) {
        return None;
    }
    let m: crate::marker::Marker = p.start();
    p.bump_any();
    Some(m.complete(p, SyntaxKind::KEY))
}

pub fn parse_bare_key(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.at(SyntaxKind::BARE_KEY) || p.nth_at(1, SyntaxKind::DOT) {
        return None;
    }
    let m = p.start();
    p.bump(SyntaxKind::BARE_KEY);
    Some(m.complete(p, SyntaxKind::KEY))
}

pub fn parse_value(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.at_ts(KEY_FIRST) {
        return None;
    }
    let m: crate::marker::Marker = p.start();
    p.bump_any();
    Some(m.complete(p, SyntaxKind::KEY))
}
