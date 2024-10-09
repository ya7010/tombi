use logos::Logos;
use rowan::GreenNode;

#[macro_use]
mod macros;

pub fn parse(source: &str) -> Parse {
    let mut parser = Parser::new(source);
    let _ = with_node!(parser.builder, lexer::Token::ROOT, parser.parse_root());

    Parse {
        green_node: parser.builder.finish(),
        errors: parser.errors,
    }
}

struct Parser<'p> {
    current_token: Option<lexer::Token>,
    pub builder: rowan::GreenNodeBuilder<'p>,
    pub lexer: logos::Lexer<'p, lexer::Token>,
    pub errors: Vec<crate::Error>,
}

impl<'p> Parser<'p> {
    fn new(source: &'p str) -> Self {
        Parser {
            current_token: None,
            lexer: lexer::Token::lexer(source),
            builder: Default::default(),
            errors: Default::default(),
        }
    }

    fn parse_root(&mut self) {}
}

#[derive(Debug, Clone)]
pub struct Parse {
    pub green_node: GreenNode,
    pub errors: Vec<crate::Error>,
}
