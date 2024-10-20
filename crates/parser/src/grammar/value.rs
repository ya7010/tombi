use super::{array::parse_array, inline_table::parse_inline_table};
use crate::{parser::Parser, token_set::TokenSet};
use syntax::{SyntaxKind::*, T};

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
            m.complete(p, VALUE);
        }
        T!('[') => parse_array(p),
        T!('{') => parse_inline_table(p),
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
