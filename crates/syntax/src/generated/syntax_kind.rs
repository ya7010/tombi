//! Generated file, do not edit by hand, see `xtask/src/codegen`

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxToken {
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
    ERROR_TOKEN,
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
    ARRAY,
    TABLE,
    INLINE_TABLE,
    ARRAY_OF_TABLE,
}
use self::SyntaxToken::*;
impl SyntaxToken {
    pub fn is_keyword(self) -> bool {
        match self {
            TRUE_KW | FALSE_KW => true,
            _ => false,
        }
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [,] => { $ crate :: SyntaxKind :: COMMA } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [=] => { $ crate :: SyntaxKind :: EQUAL } ; ['['] => { $ crate :: SyntaxKind :: BRACKET_START } ; [']'] => { $ crate :: SyntaxKind :: BRACKET_END } ; ['{'] => { $ crate :: SyntaxKind :: BRACE_START } ; ['}'] => { $ crate :: SyntaxKind :: BRACE_END } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; }
