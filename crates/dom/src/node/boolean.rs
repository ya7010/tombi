#[derive(Debug, Clone)]
pub struct BooleanNode<'a> {
    pub value: Option<bool>,
    pub syntax: &'a lexer::SyntaxElement,
    pub errors: Vec<crate::Error>,
}

impl<'a> crate::FromSyntax<'a> for BooleanNode<'a> {
    fn from_syntax(syntax: &'a lexer::SyntaxElement) -> Self {
        use lexer::Token::*;

        let mut errors = Vec::new();
        match syntax.kind() {
            BOOLEAN => {
                let value = syntax.as_token().and_then(|t| t.text().parse().ok());
                if value.is_none() {
                    errors.push(crate::Error::InvalidBooleanValue {
                        syntax: syntax.clone(),
                    });
                }
                Self {
                    value,
                    syntax,
                    errors,
                }
            }
            _ => {
                errors.push(crate::Error::UnexpectedSyntax {
                    syntax: syntax.clone(),
                });

                Self {
                    value: None,
                    syntax,
                    errors,
                }
            }
        }
    }
}
