mod container;
mod error;

pub use error::Error;
pub use lexer::Token;
use logos::Logos;

pub fn parse(source: &str) -> Parse {
    use lexer::Token::*;
    let mut lexer = lexer::Token::lexer(source);
    let mut builder = rowan::GreenNodeBuilder::default();
    let mut errors = vec![];
    builder.start_node(ROOT.into());
    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => match token {
                ROOT => {
                    unreachable!("unexpected root token");
                }
                COMMENT => {
                    // TODO: need allowed_comment_chars
                    builder.token(token.into(), lexer.slice());
                }
                NEWLINE => {
                    builder.token(token.into(), lexer.slice());
                }
                BARE_KEY => {
                    let value = lexer.slice();
                    builder.token(token.into(), value);
                }
                EQUAL => {
                    builder.token(token.into(), lexer.slice());
                }
                BASIC_STRING
                | MULTI_LINE_BASIC_STRING
                | LITERAL_STRING
                | MULTI_LINE_LITERAL_STRING
                | INTEGER
                | INTEGER_HEX
                | INTEGER_OCT
                | INTEGER_BIN
                | FLOAT
                | BOOLEAN
                | OFFSET_DATE_TIME
                | LOCAL_DATE_TIME
                | LOCAL_DATE
                | LOCAL_TIME => {
                    let value = lexer.slice();
                    builder.token(token.into(), value);
                }
                _ => continue,
            },
            Err(error) => {
                let span = lexer.span();
                errors.push(crate::Error::InvalidToken { error, span });
            }
        }
    }

    builder.finish_node();

    Parse {
        green_node: builder.finish(),
        errors: vec![],
    }
}

#[derive(Debug, Clone)]
pub struct Parse {
    pub green_node: rowan::GreenNode,
    pub errors: Vec<crate::Error>,
}

impl Parse {
    /// Turn the parse into a syntax node.
    pub fn into_syntax(self) -> lexer::SyntaxNode {
        lexer::SyntaxNode::new_root(self.green_node)
    }
}
