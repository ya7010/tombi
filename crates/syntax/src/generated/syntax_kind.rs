//! Generated file, do not edit by hand, see `xtask/src/codegen`

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    COMMA,
    DOT,
    EQUAL,
    BRACKET_START,
    BRACKET_END,
    BRACE_START,
    BRACE_END,
    TRUE_KW,
    FALSE_KW,
    BASIC_STRING,
    MULTI_LINE_BASIC_STRING,
    LITERAL_STRING,
    MULTI_LINE_LITERAL_STRING,
    INTEGER_DEC,
    INTEGER_HEX,
    INTEGER_OCT,
    INTEGER_BIN,
    FLOAT,
    BOOLEAN,
    OFFSET_DATE_TIME,
    LOCAL_DATE_TIME,
    LOCAL_DATE,
    LOCAL_TIME,
    DOTTED_KEY,
    NEWLINE,
    WHITESPACE,
    BARE_KEY,
    COMMENT,
    ROOT,
    QUOTED_KEY,
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
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        match self {
            TRUE_KW | FALSE_KW => true,
            _ => false,
        }
    }
}
impl From<SyntaxKind> for rowan::SyntaxKind {
    #[inline]
    fn from(k: SyntaxKind) -> Self {
        Self(k as u16)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [,] => { $ crate :: SyntaxKind :: COMMA } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [=] => { $ crate :: SyntaxKind :: EQUAL } ; ['['] => { $ crate :: SyntaxKind :: BRACKET_START } ; [']'] => { $ crate :: SyntaxKind :: BRACKET_END } ; ['{'] => { $ crate :: SyntaxKind :: BRACE_START } ; ['}'] => { $ crate :: SyntaxKind :: BRACE_END } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [bare_key] => { $ crate :: SyntaxKind :: BARE_KEY } ; [basic_string] => { $ crate :: SyntaxKind :: BASIC_STRING } ; [multi_line_basic_string] => { $ crate :: SyntaxKind :: MULTI_LINE_BASIC_STRING } ; [literal_string] => { $ crate :: SyntaxKind :: LITERAL_STRING } ; [multi_line_literal_string] => { $ crate :: SyntaxKind :: MULTI_LINE_LITERAL_STRING } ; [integer_dec] => { $ crate :: SyntaxKind :: INTEGER_DEC } ; [integer_hex] => { $ crate :: SyntaxKind :: INTEGER_HEX } ; [integer_oct] => { $ crate :: SyntaxKind :: INTEGER_OCT } ; [integer_bin] => { $ crate :: SyntaxKind :: INTEGER_BIN } ; [float] => { $ crate :: SyntaxKind :: FLOAT } ; [offset_date_time] => { $ crate :: SyntaxKind :: OFFSET_DATE_TIME } ; [local_date_time] => { $ crate :: SyntaxKind :: LOCAL_DATE_TIME } ; [local_date] => { $ crate :: SyntaxKind :: LOCAL_DATE } ; [local_time] => { $ crate :: SyntaxKind :: LOCAL_TIME } ; }
