use logos::Logos;
use rowan::GreenNode;

pub struct Parser<'p> {
    current_token: Option<lexer::Token>,
    pub builder: rowan::GreenNodeBuilder<'p>,
    pub lexer: logos::Lexer<'p, lexer::Token>,
    pub errors: Vec<crate::Error>,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Parser {
            current_token: None,
            lexer: lexer::Token::lexer(source),
            builder: Default::default(),
            errors: Default::default(),
        }
    }

    pub fn parse_root(&mut self) {}
}

#[derive(Debug, Clone)]
pub struct Parse {
    pub green_node: GreenNode,
    pub errors: Vec<crate::Error>,
}
