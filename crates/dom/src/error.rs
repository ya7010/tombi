#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("the syntax was invalid: {syntax:#?}")]
    InvalidSyntax { syntax: syntax::SyntaxElement },

    #[error("the boolean value was invalid: {syntax:#?}")]
    InvalidBooleanValue { syntax: syntax::SyntaxElement },

    #[error("the string value was invalid: {syntax:#?}")]
    InvalidStringValue { syntax: syntax::SyntaxElement },

    #[error("the string kind was invalid: {syntax:#?}")]
    InvalidStringKind { syntax: syntax::SyntaxElement },
}
