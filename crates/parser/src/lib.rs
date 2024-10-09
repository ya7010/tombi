mod error;
mod lang;
pub mod parser;
mod syntax_kind;

pub use error::Error;
pub use lang::TomlLang;
pub use syntax_kind::SyntaxKind;
