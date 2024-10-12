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
pub const LITERALS: &[&str] = &["JSON_STRING_LITERAL", "JSON_NUMBER_LITERAL"];
pub const TOKENS: &[&str] = &[
    "ERROR_TOKEN",
    "NEWLINE",
    "WHITESPACE",
    "IDENT",
    "COMMENT",
    "MULTILINE_COMMENT",
];

pub const NODES: &[&str] = &[
    "JSON_ROOT",
    "JSON_NUMBER_VALUE",
    "JSON_STRING_VALUE",
    "JSON_BOOLEAN_VALUE",
    "JSON_NULL_VALUE",
    "JSON_ARRAY_VALUE",
    "JSON_OBJECT_VALUE",
    "JSON_MEMBER_LIST",
    "JSON_MEMBER",
    "JSON_MEMBER_NAME",
    "JSON_ARRAY_ELEMENT_LIST",
    // Bogus nodes
    "JSON_BOGUS",
    "JSON_BOGUS_MEMBER_NAME",
    "JSON_BOGUS_VALUE",
];
