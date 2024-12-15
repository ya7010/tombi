use crate::support::float::try_from_float;

#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    value: f64,
    node: ast::Float,
}

impl Float {
    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[inline]
    pub fn node(&self) -> &ast::Float {
        &self.node
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.node.token().unwrap().range()
    }

    #[inline]
    pub fn symbol_range(&self) -> text::Range {
        self.range()
    }
}

impl TryFrom<ast::Float> for Float {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::Float) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.range();
        match try_from_float(token.text()) {
            Ok(value) => Ok(Self { value, node }),
            Err(error) => Err(vec![crate::Error::ParseFloatError { error, range }]),
        }
    }
}
