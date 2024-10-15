use crate::parser::Parser;

use super::key_value::parse_key_value;

pub fn parse(p: &mut Parser<'_>) {
    parse_key_value(p);
}
