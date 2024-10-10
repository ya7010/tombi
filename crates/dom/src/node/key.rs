#[derive(Debug, Clone)]
pub struct BareKeyNode<'a> {
    pub value: &'a str,
    pub syntax: &'a lexer::SyntaxElement,
}

#[derive(Debug, Clone)]
pub struct QuotedKeyNode<'a> {
    pub value: &'a str,
    pub syntax: &'a lexer::SyntaxElement,
}

#[derive(Debug, Clone)]
pub enum SingleKeyNode<'a> {
    Bare(BareKeyNode<'a>),
    Quoted(QuotedKeyNode<'a>),
}

#[derive(Debug, Clone)]
pub struct DottedKeysNode<'a> {
    pub value: Vec<SingleKeyNode<'a>>,
    pub syntax: &'a lexer::SyntaxElement,
}

#[derive(Debug, Clone)]
pub enum KeyNode<'a> {
    Bare(BareKeyNode<'a>),
    Quoted(QuotedKeyNode<'a>),
    Dotted(DottedKeysNode<'a>),
}

impl<'a> crate::TryFromSyntax<'a> for KeyNode<'a> {
    fn try_from_syntax(syntax: &'a lexer::SyntaxElement) -> Result<Self, Vec<crate::Error>> {
        use lexer::Token::*;

        if let Some(value) = syntax.as_token().map(|t| t.text()) {
            Ok(Self {
                kind,
                value,
                syntax,
            })
        } else {
            Err(vec![crate::Error::InvalidStringValue {
                syntax: syntax.clone(),
            }])
        }
    }
}
