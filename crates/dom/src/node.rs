mod boolean;
mod invalid;
mod string;

pub use boolean::BooleanNode;
pub use invalid::InvalidNode;
pub use string::StringNode;

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Boolean(BooleanNode<'a>),
    String(StringNode<'a>),
    Invalid(InvalidNode<'a>),
}

impl<'a> crate::FromSyntax<'a> for Node<'a> {
    fn from_syntax(syntax: &'a lexer::SyntaxElement) -> Self {
        use lexer::Token::*;

        match syntax.kind() {
            BOOLEAN => Node::Boolean(BooleanNode::from_syntax(syntax)),
            BASIC_STRING => Node::String(StringNode::from_syntax(syntax)),
            _ => Node::Invalid(InvalidNode::from_syntax(syntax)),
        }
    }
}
