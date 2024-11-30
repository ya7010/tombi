use super::Value;

#[derive(Debug)]
pub struct Array {
    range: text::Range,
    values: Vec<Value>,
}

impl Array {
    pub fn range(&self) -> text::Range {
        self.range
    }

    pub fn values(&self) -> &[Value] {
        &self.values
    }
}

impl From<ast::Array> for Array {
    fn from(node: ast::Array) -> Self {
        Self {
            range: node.bracket_start().unwrap().text_range()
                + node.bracket_end().unwrap().text_range(),
            values: node.values().map(Into::into).collect(),
        }
    }
}
