#![allow(dead_code)]

pub const PUNCTUATIONS: &[(&str, &str)] = &[
    (",", "COMMA"),
    (".", "DOT"),
    ("=", "EQUAL"),
    ("[", "BRACKET_START"),
    ("]", "BRACKET_END"),
    ("{", "BRACE_START"),
    ("}", "BRACE_END"),
];

pub const KEYWORDS: &[&str] = &["true", "false"];
pub const LITERALS: &[&str] = &[
    "BASIC_STRING",
    "MULTI_LINE_BASIC_STRING",
    "LITERAL_STRING",
    "MULTI_LINE_LITERAL_STRING",
    "INTEGER_DEC",
    "INTEGER_HEX",
    "INTEGER_OCT",
    "INTEGER_BIN",
    "FLOAT",
    "BOOLEAN",
    "OFFSET_DATE_TIME",
    "LOCAL_DATE_TIME",
    "LOCAL_DATE",
    "LOCAL_TIME",
];
pub const TOKENS: &[&str] = &["NEWLINE", "WHITESPACE", "BARE_KEY", "COMMENT"];

pub const NODES: &[&str] = &[
    "ROOT",
    "QUOTED_KEY",
    "DOTTED_KEYS",
    "KEY",
    "VALUE",
    "KEY_VALUE",
    "ARRAY",
    "TABLE",
    "INLINE_TABLE",
    "ARRAY_OF_TABLE",
];
