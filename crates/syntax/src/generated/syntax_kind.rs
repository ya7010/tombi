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
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
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
macro_rules ! T { [","] => { $ crate :: SyntaxKind :: COMMA } ; ["."] => { $ crate :: SyntaxKind :: DOT } ; ["="] => { $ crate :: SyntaxKind :: EQUAL } ; ["["] => { $ crate :: SyntaxKind :: BRACKET_START } ; ["]"] => { $ crate :: SyntaxKind :: BRACKET_END } ; ["{"] => { $ crate :: SyntaxKind :: BRACE_START } ; ["}"] => { $ crate :: SyntaxKind :: BRACE_END } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [EOF] => { $ crate :: SyntaxKind :: EOF } ; }
