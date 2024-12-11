#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    value: f64,
    range: text::Range,
    symbol_range: text::Range,
}

impl Float {
    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }

    #[inline]
    pub fn symbol_range(&self) -> text::Range {
        self.symbol_range
    }
}

impl TryFrom<ast::Float> for Float {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::Float) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.range();
        match token.text().parse() {
            Ok(value) => Ok(Self {
                value,
                range,
                symbol_range: range,
            }),
            Err(error) => Err(vec![crate::Error::ParseFloatError { error, range }]),
        }
    }
}
