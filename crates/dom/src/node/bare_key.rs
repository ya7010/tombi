#[derive(Debug, Clone)]
pub struct BareKeyNode<'a> {
    pub value: &'a str,
    pub syntax: &'a lexer::SyntaxElement,
}

impl<'a> crate::TryFromSyntax<'a> for BareKeyNode<'a> {
    fn try_from_syntax(syntax: &'a lexer::SyntaxElement) -> Result<Self, Vec<crate::Error>> {
        use lexer::Token::*;
        if syntax.kind() != BARE_KEY {
            unreachable!("invalid bare key kind: {syntax:#?}")
        }
        if let Some(value) = syntax.as_token().map(|t| t.text()) {
            Ok(Self { value, syntax })
        } else {
            Err(vec![crate::Error::InvalidStringValue {
                syntax: syntax.clone(),
            }])
        }
    }
}
