#[derive(Debug, Clone)]
pub struct BooleanNode<'a> {
    pub value: bool,
    pub syntax: &'a syntax::SyntaxElement,
}

impl<'a> crate::TryFromSyntax<'a> for BooleanNode<'a> {
    fn try_from_syntax(syntax: &'a syntax::SyntaxElement) -> Result<Self, Vec<crate::Error>> {
        use syntax::Token::*;

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
