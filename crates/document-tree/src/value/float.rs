#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    value: f64,
    range: text::Range,
}

impl Float {
    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl TryFrom<ast::Float> for Float {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::Float) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.range();
        match token.text().parse() {
            Ok(value) => Ok(Self { value, range }),
            Err(error) => Err(vec![crate::Error::ParseFloatError { error, range }]),
        }
    }
}
