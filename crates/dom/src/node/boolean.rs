#[derive(Debug, Clone)]
pub struct BooleanNode<'a> {
    pub value: bool,
    pub syntax: &'a lexer::SyntaxElement,
}

impl<'a> crate::FromSyntax<'a> for BooleanNode<'a> {
    fn from_syntax(syntax: &'a lexer::SyntaxElement) -> Result<Self, Vec<crate::Error>> {
        use lexer::Token::*;

        match syntax.kind() {
            BOOLEAN => {
                if let Some(value) = syntax.as_token().and_then(|t| t.text().parse().ok()) {
                    Ok(Self { value, syntax })
                } else {
                    Err(vec![crate::Error::InvalidBooleanValue {
                        syntax: syntax.clone(),
                    }])
                }
            }
            _ => unreachable!("invalid boolean kind: {syntax:#?}"),
        }
    }
}
