use quote::{format_ident, quote};

pub const PUNCTUATIONS: &[(&str, &str)] = &[
    (",", "COMMA"),
    (".", "DOT"),
    ("=", "EQUAL"),
    ("[", "BRACKET_START"),
    ("]", "BRACKET_END"),
    ("{", "BRACE_START"),
    ("}", "BRACE_END"),
    ("[[", "DOUBLE_BRACKET_START"),
    ("]]", "DOUBLE_BRACKET_END"),
];

pub const KEYWORDS: &[&str] = &["true", "false"];

pub const LITERALS: &[RegexItem] = &[
    RegexItem::new_with_callback(
        "BASIC_STRING",
        r#"""#,
        r#"|lex| lex_single_line_string(lex, '"')"#,
    ),
    RegexItem::new_with_callback(
        "MULTI_LINE_BASIC_STRING",
        r#"""""#,
        r#"|lex| lex_multi_line_string(lex, '"')"#,
    ),
    RegexItem::new_with_callback(
        "LITERAL_STRING",
        r#"'"#,
        r#"|lex| lex_single_line_string(lex, '\'')"#,
    ),
    RegexItem::new_with_callback(
        "MULTI_LINE_LITERAL_STRING",
        r#"'''"#,
        r#"|lex| lex_multi_line_string(lex, '\'')"#,
    ),
    RegexItem::new_with_priority("INTEGER_DEC", r"[+-]?[0-9_]+", 4),
    RegexItem::new("INTEGER_HEX", r"0x[0-9A-Fa-f_]+"),
    RegexItem::new("INTEGER_OCT", r"0o[0-7_]+"),
    RegexItem::new("INTEGER_BIN", r"0b(0|1|_)+"),
    RegexItem::new_with_priority(
        "FLOAT",
        r"[-+]?([0-9_]+(\.[0-9_]+)?([eE][+-]?[0-9_]+)?|nan|inf)",
        3,
    ),
    RegexItem::new("BOOLEAN", r"true|false"),
    RegexItem::new(
        "OFFSET_DATE_TIME",
        r#"(?:[1-9]\d\d\d-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)(?:T|t| )(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:(?:\.|,)\d+)?(?:[Zz]|[+-][01]\d:[0-5]\d)"#,
    ),
    RegexItem::new(
        "LOCAL_DATE_TIME",
        r#"(?:[1-9]\d\d\d-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)(?:T|t| )(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:(?:\.|,)\d+)?"#,
    ),
    RegexItem::new(
        "LOCAL_DATE",
        r#"(?:[1-9]\d\d\d-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)"#,
    ),
    RegexItem::new(
        "LOCAL_TIME",
        r#"(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:(?:\.|,)\d+)?"#,
    ),
];
pub const TOKENS: &[RegexItem] = &[
    RegexItem::new("WHITESPACE", r"[ \t]+"),
    RegexItem::new("NEWLINE", r"(\n|\r\n)+"),
    RegexItem::new_with_priority("BARE_KEY", r"[A-Za-z0-9_-]+", 2),
    RegexItem::new("COMMENT", r"#[^\n\r]*"),
];

pub const NODES: &[&str] = &[
    "ROOT",
    "QUOTED_KEY",
    "DOTTED_KEY",
    "DOTTED_KEYS",
    "KEY",
    "VALUE",
    "KEY_VALUE",
    "STRING",
    "INTEGER",
    "ARRAY",
    "ARRAY_ELEMENT",
    "TABLE",
    "INLINE_TABLE",
    "INLINE_TABLE_ELEMENT_LIST",
    "ARRAY_OF_TABLE",
];

#[derive(Debug)]
pub struct RegexItem<'a> {
    pub name: &'a str,
    regex: &'a str,
    callback: Option<&'a str>,
    priority: Option<u8>,
}

impl<'a> RegexItem<'a> {
    pub const fn new(name: &'a str, regex: &'a str) -> Self {
        Self {
            name,
            regex,
            callback: None,
            priority: None,
        }
    }

    pub const fn new_with_callback(name: &'a str, regex: &'a str, callback: &'a str) -> Self {
        Self {
            name,
            regex,
            callback: Some(callback),
            priority: None,
        }
    }

    pub const fn new_with_priority(name: &'a str, regex: &'a str, priority: u8) -> Self {
        Self {
            name,
            regex,
            callback: None,
            priority: Some(priority),
        }
    }

    pub fn to_attr_token(&self) -> proc_macro2::TokenStream {
        let name = format_ident!("{}", self.name);
        let regex = self.regex;
        if let Some(priority) = self.priority {
            let priority = proc_macro2::Literal::u8_unsuffixed(priority);
            quote! { #[regex(#regex, priority = #priority)] #name }
        } else if let Some(callback) = &self.callback {
            let callback: proc_macro2::TokenStream = (&*(*callback)).parse().unwrap();

            quote! { #[regex(#regex, callback = #callback)] #name }
        } else {
            quote! { #[regex(#regex)] #name }
        }
    }
}
