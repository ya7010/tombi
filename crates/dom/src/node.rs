mod bare_key;
mod boolean;
mod string;

pub use bare_key::BareKeyNode;
pub use boolean::BooleanNode;
pub use string::StringNode;

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Boolean(BooleanNode<'a>),
    String(StringNode<'a>),
    BareKey(BareKeyNode<'a>),
}

impl<'a> crate::TryFromSyntax<'a> for Node<'a> {
    fn try_from_syntax(syntax: &'a syntax::SyntaxElement) -> Result<Self, Vec<crate::Error>> {
        use syntax::SyntaxKind::*;

        match syntax.kind() {
            BOOLEAN => BooleanNode::try_from_syntax(syntax).map(|node| Node::Boolean(node)),
            BASIC_STRING | MULTI_LINE_BASIC_STRING | LITERAL_STRING | MULTI_LINE_LITERAL_STRING => {
                StringNode::try_from_syntax(syntax).map(|node| Node::String(node))
            }
            BARE_KEY => BareKeyNode::try_from_syntax(syntax).map(|node| Node::BareKey(node)),
            _ => Err(vec![crate::Error::InvalidSyntax {
                syntax: syntax.clone(),
            }]),
        }
    }
}
