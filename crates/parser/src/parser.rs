use logos::Logos;
use rowan::GreenNode;

pub struct Parser<'p> {
    pub builder: rowan::GreenNodeBuilder<'p>,
    pub lexer: logos::Lexer<'p, lexer::Token>,
    pub errors: Vec<crate::Error>,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Parser {
            lexer: lexer::Token::lexer(source),
            builder: Default::default(),
            errors: Default::default(),
        }
    }

    pub fn parse_root(&mut self) {
        use lexer::Token::*;

        while let Some(token) = self.lexer.next() {
            match token {
                Ok(token) => match token {
                    COMMENT => {
                        // TODO: need allowed_comment_chars
                        self.builder.token(token.into(), self.lexer.slice());
                    }
                    _ => continue,
                },
                Err(error) => {
                    let span = self.lexer.span();
                    self.errors.push(crate::Error::InvalidToken { error, span });
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parse {
    pub green_node: GreenNode,
    pub errors: Vec<crate::Error>,
}
