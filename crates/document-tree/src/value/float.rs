use toml_version::TomlVersion;

use crate::{
    support::float::try_from_float, DocumentTreeResult, IntoDocumentTreeResult, ValueImpl,
    ValueType,
};

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

impl ValueImpl for Float {
    fn value_type(&self) -> ValueType {
        ValueType::Float
    }

    fn range(&self) -> text::Range {
        self.range()
    }
}

impl IntoDocumentTreeResult<crate::Value> for ast::Float {
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

        match try_from_float(token.text()) {
            Ok(value) => DocumentTreeResult {
                tree: crate::Value::Float(crate::Float { value, node: self }),
                errors: Vec::with_capacity(0),
            },
            Err(error) => DocumentTreeResult {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::ParseFloatError { error, range }],
            },
        }
    }
}
