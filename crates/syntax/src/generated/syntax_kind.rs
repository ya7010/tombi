//! Generated file, do not edit by hand, see `xtask/src/codegen`

#[doc = r" The kind of syntax node, e.g. `WHITESPACE`, `COMMENT`, or `TABLE`."]
#[derive(logos :: Logos, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
# [logos (error = crate :: Error)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    #[token(",")]
    COMMA,
    #[token(".")]
    DOT,
    #[token("=")]
    EQUAL,
    #[token("[")]
    BRACKET_START,
    #[token("]")]
    BRACKET_END,
    #[token("{")]
    BRACE_START,
    #[token("}")]
    BRACE_END,
    #[token("[[", priority = 2)]
    DOUBLE_BRACKET_START,
    #[token("]]", priority = 2)]
    DOUBLE_BRACKET_END,
    # [regex ("\"" , callback = | lex | lex_single_line_string (lex , '"'))]
    BASIC_STRING,
    # [regex ("\"\"\"" , callback = | lex | lex_multi_line_string (lex , '"'))]
    MULTI_LINE_BASIC_STRING,
    # [regex ("'" , callback = | lex | lex_single_line_string (lex , '\''))]
    LITERAL_STRING,
    # [regex ("'''" , callback = | lex | lex_multi_line_string (lex , '\''))]
    MULTI_LINE_LITERAL_STRING,
    #[regex("[+-]?[0-9_]+", priority = 4)]
    INTEGER_DEC,
    #[regex("0x[0-9A-Fa-f_]+")]
    INTEGER_HEX,
    #[regex("0o[0-7_]+")]
    INTEGER_OCT,
    #[regex("0b(0|1|_)+")]
    INTEGER_BIN,
    #[regex("[-+]?([0-9_]+(\\.[0-9_]+)?([eE][+-]?[0-9_]+)?|nan|inf)", priority = 3)]
    FLOAT,
    #[regex("true|false")]
    BOOLEAN,
    # [regex ("(?:[1-9]\\d\\d\\d-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)(?:T|t| )(?:[01]\\d|2[0-3]):[0-5]\\d:[0-5]\\d(?:(?:\\.|,)\\d+)?(?:[Zz]|[+-][01]\\d:[0-5]\\d)")]
    OFFSET_DATE_TIME,
    # [regex ("(?:[1-9]\\d\\d\\d-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)(?:T|t| )(?:[01]\\d|2[0-3]):[0-5]\\d:[0-5]\\d(?:(?:\\.|,)\\d+)?")]
    LOCAL_DATE_TIME,
    # [regex ("(?:[1-9]\\d\\d\\d-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)")]
    LOCAL_DATE,
    #[regex("(?:[01]\\d|2[0-3]):[0-5]\\d:[0-5]\\d(?:(?:\\.|,)\\d+)?")]
    LOCAL_TIME,
    #[regex("[ \\t]+")]
    WHITESPACE,
    #[regex("(\\n|\\r\\n)+")]
    NEWLINE,
    #[regex("[A-Za-z0-9_-]+", priority = 2)]
    BARE_KEY,
    #[regex("#[^\\n\\r]*")]
    COMMENT,
    ERROR,
    ROOT,
    QUOTED_KEY,
    DOTTED_KEY,
    DOTTED_KEYS,
    KEY,
    VALUE,
    KEY_VALUE,
    STRING,
    INTEGER,
    ARRAY,
    ARRAY_ELEMENT,
    TABLE,
    INLINE_TABLE,
    INLINE_TABLE_ELEMENT_LIST,
    ARRAY_OF_TABLE,
    #[doc(hidden)]
    INVALID_TOKENS,
    #[doc(hidden)]
    __LAST,
}
impl SyntaxKind {
    #[inline]
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::WHITESPACE)
    }
}
impl From<SyntaxKind> for rowan::SyntaxKind {
    #[inline]
    fn from(k: SyntaxKind) -> Self {
        Self(k as u16)
    }
}
impl From<u16> for SyntaxKind {
    #[inline]
    fn from(d: u16) -> SyntaxKind {
        assert!(d <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
    }
}
impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(k: SyntaxKind) -> u16 {
        k as u16
    }
}
fn lex_single_line_string(lex: &mut logos::Lexer<SyntaxKind>, quote: char) -> bool {
    let remainder: &str = lex.remainder();
    let mut total_len = 0;
    for c in remainder.chars() {
        total_len += c.len_utf8();
        if c == quote {
            lex.bump(remainder[0..total_len].as_bytes().len());
            return true;
        }
    }
    false
}
fn lex_multi_line_string(lex: &mut logos::Lexer<SyntaxKind>, quote: char) -> bool {
    let remainder: &str = lex.remainder();
    let mut total_len = 0;
    let mut quote_count = 0;
    let mut escaped = false;
    let mut quotes_found = false;
    for c in remainder.chars() {
        if quotes_found {
            if c != quote {
                if quote_count >= 6 {
                    return false;
                }
                lex.bump(remainder[0..total_len].as_bytes().len());
                return true;
            } else {
                quote_count += 1;
                total_len += c.len_utf8();
                continue;
            }
        }
        total_len += c.len_utf8();
        if c == '\\' {
            escaped = true;
            continue;
        }
        if c == quote && !escaped {
            quote_count += 1;
        } else {
            quote_count = 0;
        }
        if quote_count == 3 {
            quotes_found = true;
        }
        escaped = false;
    }
    if quotes_found {
        if quote_count >= 6 {
            return false;
        }
        lex.bump(remainder[0..total_len].as_bytes().len());
        true
    } else {
        false
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [,] => { $ crate :: SyntaxKind :: COMMA } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [=] => { $ crate :: SyntaxKind :: EQUAL } ; ['['] => { $ crate :: SyntaxKind :: BRACKET_START } ; [']'] => { $ crate :: SyntaxKind :: BRACKET_END } ; ['{'] => { $ crate :: SyntaxKind :: BRACE_START } ; ['}'] => { $ crate :: SyntaxKind :: BRACE_END } ; ["[["] => { $ crate :: SyntaxKind :: DOUBLE_BRACKET_START } ; ["]]"] => { $ crate :: SyntaxKind :: DOUBLE_BRACKET_END } ; [bare_key] => { $ crate :: SyntaxKind :: BARE_KEY } ; [basic_string] => { $ crate :: SyntaxKind :: BASIC_STRING } ; [multi_line_basic_string] => { $ crate :: SyntaxKind :: MULTI_LINE_BASIC_STRING } ; [literal_string] => { $ crate :: SyntaxKind :: LITERAL_STRING } ; [multi_line_literal_string] => { $ crate :: SyntaxKind :: MULTI_LINE_LITERAL_STRING } ; [integer_dec] => { $ crate :: SyntaxKind :: INTEGER_DEC } ; [integer_hex] => { $ crate :: SyntaxKind :: INTEGER_HEX } ; [integer_oct] => { $ crate :: SyntaxKind :: INTEGER_OCT } ; [integer_bin] => { $ crate :: SyntaxKind :: INTEGER_BIN } ; [float] => { $ crate :: SyntaxKind :: FLOAT } ; [offset_date_time] => { $ crate :: SyntaxKind :: OFFSET_DATE_TIME } ; [local_date_time] => { $ crate :: SyntaxKind :: LOCAL_DATE_TIME } ; [local_date] => { $ crate :: SyntaxKind :: LOCAL_DATE } ; [local_time] => { $ crate :: SyntaxKind :: LOCAL_TIME } ; }
