mod error;
mod lang;
pub mod syntax;
mod token;

pub use error::Error;
pub use lang::TomlLang;
pub use syntax::*;
pub use token::Token;
