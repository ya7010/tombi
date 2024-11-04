use quote::{format_ident, quote};

pub const PUNCTUATIONS: &[PunctuationItem] = &[
    PunctuationItem::new(",", "COMMA"),
    PunctuationItem::new(".", "DOT"),
    PunctuationItem::new("=", "EQUAL"),
    PunctuationItem::new("[", "BRACKET_START"),
    PunctuationItem::new("]", "BRACKET_END"),
    PunctuationItem::new("{", "BRACE_START"),
    PunctuationItem::new("}", "BRACE_END"),
    PunctuationItem::new_without_attribute("[[", "DOUBLE_BRACKET_START"),
    PunctuationItem::new_without_attribute("]]", "DOUBLE_BRACKET_END"),
];

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
    RegexItem::new("INTEGER_BIN", r"0b[0|1|_]+"),
    RegexItem::new_with_priority(
        "FLOAT",
        r"[-+]?(:?[0-9_]+(:?\.[0-9_]+)?(:?[eE][+-]?[0-9_]+)?|nan|inf)",
        3,
    ),
    RegexItem::new("BOOLEAN", r"true|false"),
    RegexItem::new_with_priority(
        "OFFSET_DATE_TIME",
        r#"\d{4}-\d{2}-\d{2}[Tt ]\d{2}:\d{2}:\d{2}(?:[\.,]\d+)?(?:[Zz]|[+-]\d{2}:\d{2})"#,
        5,
    ),
    RegexItem::new_with_priority(
        "LOCAL_DATE_TIME",
        r#"\d{4}-\d{2}-\d{2}(?:T|t| )\d{2}:\d{2}:\d{2}(?:[\.,]\d+)?"#,
        5,
    ),
    RegexItem::new_with_priority("LOCAL_DATE", r#"\d{4}-\d{2}-\d{2}"#, 5),
    RegexItem::new_with_priority("LOCAL_TIME", r#"\d{2}:\d{2}:\d{2}(?:[\.,]\d+)?"#, 5),
];
pub const TOKENS: &[TokenItem] = &[
    TokenItem::Regex(RegexItem::new("WHITESPACE", r"[ \t]+")),
    TokenItem::Regex(RegexItem::new("NEWLINE", r"\n|\r\n")),
    TokenItem::Regex(RegexItem::new_with_priority(
        "BARE_KEY",
        r"[A-Za-z0-9_-]+",
        2,
    )),
    TokenItem::Regex(RegexItem::new("COMMENT", r"#[^\n\r]*")),
    TokenItem::Token("ERROR"),
];

pub const NODES: &[&str] = &[
    "ROOT",
    "DOTTED_KEYS",
    "KEYS",
    "KEY",
    "VALUE",
    "KEY_VALUE",
    "ARRAY",
    "TABLE",
    "INLINE_TABLE",
    "ARRAY_OF_TABLE",
];

#[derive(Debug)]
pub struct PunctuationItem<'a> {
    pub token: &'a str,
    pub name: &'a str,
    has_attribute: bool,
}

impl<'a> PunctuationItem<'a> {
    pub const fn new(token: &'a str, name: &'a str) -> Self {
        Self {
            token,
            name,
            has_attribute: true,
        }
    }

    pub const fn new_without_attribute(token: &'a str, name: &'a str) -> Self {
        Self {
            token,
            name,
            has_attribute: false,
        }
    }

    pub fn to_attr_token(&self) -> proc_macro2::TokenStream {
        let name = format_ident!("{}", self.name);
        let token = self.token;
        if self.has_attribute {
            quote! { #[token(#token)] #name }
        } else {
            quote! { #name }
        }
    }
}

pub enum TokenItem<'a> {
    Token(&'a str),
    Regex(RegexItem<'static>),
}

impl<'a> TokenItem<'a> {
    pub fn name(&'a self) -> &'a str {
        match self {
            Self::Token(name) => name,
            Self::Regex(regex) => regex.name(),
        }
    }
    pub fn to_attr_token(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Token(name) => {
                let name = format_ident!("{}", name);
                quote! { #name }
            }
            Self::Regex(regex) => regex.to_attr_token(),
        }
    }
}

#[derive(Debug)]
pub struct RegexItem<'a> {
    name: &'a str,
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

    pub fn name(&'a self) -> &'a str {
        self.name
    }

    pub fn to_attr_token(&self) -> proc_macro2::TokenStream {
        let name = format_ident!("{}", self.name);
        let regex = self.regex;
        if let Some(priority) = self.priority {
            let priority = proc_macro2::Literal::u8_unsuffixed(priority);
            quote! { #[regex(#regex, priority = #priority)] #name }
        } else if let Some(callback) = &self.callback {
            let callback: proc_macro2::TokenStream = callback.parse().unwrap();

            quote! { #[regex(#regex, callback = #callback)] #name }
        } else {
            quote! { #[regex(#regex)] #name }
        }
    }
}
