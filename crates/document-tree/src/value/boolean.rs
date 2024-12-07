#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean {
    value: bool,
    range: text::Range,
}

impl Boolean {
    pub(crate) fn new(text: &str, range: text::Range) -> Self {
        Self {
            value: match text {
                "true" => true,
                "false" => false,
                _ => unreachable!(),
            },
            range,
        }
    }

    #[inline]
    pub fn value(&self) -> bool {
        self.value
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl TryFrom<ast::Boolean> for Boolean {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::Boolean) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Ok(Self::new(token.text(), token.text_range()))
    }
}
