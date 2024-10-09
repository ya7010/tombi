mod error;
mod parser;

pub use error::Error;
pub use lexer::Token;
pub use parser::Parse;

#[macro_use]
mod macros;

pub fn parse(source: &str) -> Parse {
    let mut parser = parser::Parser::new(source);
    let _ = with_node!(parser.builder, lexer::Token::ROOT, parser.parse_root());

    Parse {
        green_node: parser.builder.finish(),
        errors: parser.errors,
    }
}
