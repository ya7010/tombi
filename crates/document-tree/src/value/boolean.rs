#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean {
    value: bool,
    node: ast::Boolean,
}

impl Boolean {
    #[inline]
    pub fn value(&self) -> bool {
        self.value
    }

    #[inline]
    pub fn node(&self) -> &ast::Boolean {
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

impl TryFrom<ast::Boolean> for Boolean {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::Boolean) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Ok(Self {
            value: match token.text() {
                "true" => true,
                "false" => false,
                _ => unreachable!(),
            },
            node,
        })
    }
}
