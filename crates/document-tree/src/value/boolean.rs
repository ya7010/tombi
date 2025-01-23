use toml_version::TomlVersion;

use crate::{DocumentTreeResult, IntoDocumentTreeResult, ValueImpl, ValueType};

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

impl ValueImpl for Boolean {
    fn value_type(&self) -> ValueType {
        ValueType::Boolean
    }

    fn range(&self) -> text::Range {
        self.range()
    }
}

impl IntoDocumentTreeResult<crate::Value> for ast::Boolean {
    fn into_document_tree_result(
        self,
        _toml_version: TomlVersion,
    ) -> DocumentTreeResult<crate::Value> {
        let range = self.range();
        let Some(token) = self.token() else {
            return DocumentTreeResult {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::IncompleteNode { range }],
            };
        };

        let value = match token.text() {
            "true" => true,
            "false" => false,
            _ => unreachable!(),
        };

        DocumentTreeResult {
            tree: crate::Value::Boolean(crate::Boolean { value, node: self }),
            errors: Vec::with_capacity(0),
        }
    }
}
