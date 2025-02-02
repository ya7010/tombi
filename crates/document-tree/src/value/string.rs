use toml_version::TomlVersion;

use crate::{DocumentTreeAndErrors, IntoDocumentTreeAndErrors, ValueImpl, ValueType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringKind {
    BasicString(ast::BasicString),
    LiteralString(ast::LiteralString),
    MultiLineBasicString(ast::MultiLineBasicString),
    MultiLineLiteralString(ast::MultiLineLiteralString),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String {
    kind: StringKind,
    value: std::string::String,
}

impl crate::String {
    pub fn try_new(
        kind: StringKind,
        value: std::string::String,
        toml_version: TomlVersion,
    ) -> Result<Self, crate::support::string::ParseError> {
        let string = Self { kind, value };
        string.try_to_raw_string(toml_version)?;

        Ok(string)
    }

    #[inline]
    pub fn kind(&self) -> &StringKind {
        &self.kind
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub fn to_raw_string(&self, toml_version: TomlVersion) -> std::string::String {
        // NOTE: String has already been validated by `impl TryIntoDocumentTree<String>`,
        //       so it's safe to unwrap.
        self.try_to_raw_string(toml_version).unwrap()
    }

    #[inline]
    fn try_to_raw_string(
        &self,
        toml_version: TomlVersion,
    ) -> Result<std::string::String, crate::support::string::ParseError> {
        match self.kind {
            StringKind::BasicString(_) => {
                crate::support::string::try_from_basic_string(&self.value, toml_version)
            }
            StringKind::LiteralString(_) => {
                crate::support::string::try_from_literal_string(&self.value)
            }
            StringKind::MultiLineBasicString(_) => {
                crate::support::string::try_from_multi_line_basic_string(&self.value, toml_version)
            }
            StringKind::MultiLineLiteralString(_) => {
                crate::support::string::try_from_multi_line_literal_string(&self.value)
            }
        }
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        match self.kind() {
            StringKind::BasicString(node) => node.token(),
            StringKind::LiteralString(node) => node.token(),
            StringKind::MultiLineBasicString(node) => node.token(),
            StringKind::MultiLineLiteralString(node) => node.token(),
        }
        .unwrap()
        .range()
    }

    #[inline]
    pub fn symbol_range(&self) -> text::Range {
        self.range()
    }
}

impl ValueImpl for crate::String {
    fn value_type(&self) -> ValueType {
        ValueType::String
    }

    fn range(&self) -> text::Range {
        self.range()
    }
}

impl IntoDocumentTreeAndErrors<crate::Value> for ast::BasicString {
    fn into_document_tree_and_errors(
        self,
        toml_version: TomlVersion,
    ) -> DocumentTreeAndErrors<crate::Value> {
        let range = self.range();
        let Some(token) = self.token() else {
            return DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::IncompleteNode { range }],
            };
        };

        match crate::String::try_new(
            StringKind::BasicString(self),
            token.text().to_string(),
            toml_version,
        ) {
            Ok(string) => DocumentTreeAndErrors {
                tree: crate::Value::String(string),
                errors: Vec::with_capacity(0),
            },
            Err(error) => DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::ParseStringError { error, range }],
            },
        }
    }
}

impl IntoDocumentTreeAndErrors<crate::Value> for ast::LiteralString {
    fn into_document_tree_and_errors(
        self,
        toml_version: TomlVersion,
    ) -> DocumentTreeAndErrors<crate::Value> {
        let range = self.range();
        let Some(token) = self.token() else {
            return DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::IncompleteNode { range }],
            };
        };

        match crate::String::try_new(
            StringKind::LiteralString(self),
            token.text().to_string(),
            toml_version,
        ) {
            Ok(string) => DocumentTreeAndErrors {
                tree: crate::Value::String(string),
                errors: Vec::with_capacity(0),
            },
            Err(error) => DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::ParseStringError { error, range }],
            },
        }
    }
}

impl IntoDocumentTreeAndErrors<crate::Value> for ast::MultiLineBasicString {
    fn into_document_tree_and_errors(
        self,
        toml_version: TomlVersion,
    ) -> DocumentTreeAndErrors<crate::Value> {
        let range = self.range();
        let Some(token) = self.token() else {
            return DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::IncompleteNode { range }],
            };
        };

        match crate::String::try_new(
            StringKind::MultiLineBasicString(self),
            token.text().to_string(),
            toml_version,
        ) {
            Ok(string) => DocumentTreeAndErrors {
                tree: crate::Value::String(string),
                errors: Vec::with_capacity(0),
            },
            Err(error) => DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::ParseStringError { error, range }],
            },
        }
    }
}

impl IntoDocumentTreeAndErrors<crate::Value> for ast::MultiLineLiteralString {
    fn into_document_tree_and_errors(
        self,
        toml_version: TomlVersion,
    ) -> DocumentTreeAndErrors<crate::Value> {
        let range = self.range();
        let Some(token) = self.token() else {
            return DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::IncompleteNode { range }],
            };
        };

        match crate::String::try_new(
            StringKind::MultiLineLiteralString(self),
            token.text().to_string(),
            toml_version,
        ) {
            Ok(string) => DocumentTreeAndErrors {
                tree: crate::Value::String(string),
                errors: Vec::with_capacity(0),
            },
            Err(error) => DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::ParseStringError { error, range }],
            },
        }
    }
}
