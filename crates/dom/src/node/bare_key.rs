#[derive(Debug, Clone)]
pub struct BareKeyNode<'a> {
    pub value: &'a str,
    pub syntax: &'a syntax::SyntaxElement,
}

impl<'a> crate::TryFromSyntax<'a> for BareKeyNode<'a> {
    fn try_from_syntax(syntax: &'a syntax::SyntaxElement) -> Result<Self, Vec<crate::Error>> {
        use syntax::SyntaxKind::*;
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
