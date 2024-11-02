use super::{array::parse_array, inline_table::parse_inline_table, tailing_comment};
use crate::{marker::Marker, parser::Parser, token_set::TokenSet};
use syntax::{SyntaxKind::*, T};

pub fn parse_value(p: &mut Parser<'_>, m: Marker) {
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
            tailing_comment(p);
            m.complete(p, kind);
        }
        T!('[') => parse_array(p, m),
        T!('{') => parse_inline_table(p, m),
        _ => {
            while !p.at_ts(TokenSet::new(&[NEWLINE, COMMENT, EOF])) {
                p.bump_any();
            }
            p.error(crate::Error::ExpectedValue);
            tailing_comment(p);
            m.complete(p, INVALID_TOKEN);
        }
    }
}
