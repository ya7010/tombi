use crate::{
    support::chrono::{
        try_new_local_date, try_new_local_date_time, try_new_local_time, try_new_offset_date_time,
    },
    DocumentTreeAndErrors, IntoDocumentTreeAndErrors, ValueImpl, ValueType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffsetDateTime {
    value: date_time::OffsetDateTime,
    node: ast::OffsetDateTime,
}

impl OffsetDateTime {
    #[inline]
    pub fn value(&self) -> &date_time::OffsetDateTime {
        &self.value
    }

    #[inline]
    pub fn node(&self) -> &ast::OffsetDateTime {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDateTime {
    value: date_time::LocalDateTime,
    node: ast::LocalDateTime,
}

impl LocalDateTime {
    #[inline]
    pub fn value(&self) -> &date_time::LocalDateTime {
        &self.value
    }

    #[inline]
    pub fn node(&self) -> &ast::LocalDateTime {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDate {
    value: date_time::LocalDate,
    node: ast::LocalDate,
}

impl LocalDate {
    #[inline]
    pub fn value(&self) -> &date_time::LocalDate {
        &self.value
    }

    #[inline]
    pub fn node(&self) -> &ast::LocalDate {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalTime {
    value: date_time::LocalTime,
    node: ast::LocalTime,
}

impl LocalTime {
    #[inline]
    pub fn value(&self) -> &date_time::LocalTime {
        &self.value
    }

    #[inline]
    pub fn node(&self) -> &ast::LocalTime {
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

impl ValueImpl for OffsetDateTime {
    fn value_type(&self) -> ValueType {
        ValueType::OffsetDateTime
    }

    fn range(&self) -> text::Range {
        self.range()
    }
}

impl ValueImpl for LocalDateTime {
    fn value_type(&self) -> ValueType {
        ValueType::LocalDateTime
    }

    fn range(&self) -> text::Range {
        self.range()
    }
}

impl ValueImpl for LocalDate {
    fn value_type(&self) -> ValueType {
        ValueType::LocalDate
    }

    fn range(&self) -> text::Range {
        self.range()
    }
}

impl ValueImpl for LocalTime {
    fn value_type(&self) -> ValueType {
        ValueType::LocalTime
    }

    fn range(&self) -> text::Range {
        self.range()
    }
}

impl From<crate::OffsetDateTime> for date_time::OffsetDateTime {
    fn from(node: crate::OffsetDateTime) -> Self {
        node.value
    }
}

impl From<crate::LocalDateTime> for date_time::LocalDateTime {
    fn from(node: crate::LocalDateTime) -> Self {
        node.value
    }
}

impl From<crate::LocalDate> for date_time::LocalDate {
    fn from(node: crate::LocalDate) -> Self {
        node.value
    }
}

impl From<crate::LocalTime> for date_time::LocalTime {
    fn from(node: crate::LocalTime) -> Self {
        node.value
    }
}

impl IntoDocumentTreeAndErrors<crate::Value> for ast::OffsetDateTime {
    fn into_document_tree_and_errors(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> DocumentTreeAndErrors<crate::Value> {
        let range = self.range();
        let Some(_) = self.token() else {
            return DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::IncompleteNode { range }],
            };
        };

        match try_new_offset_date_time(&self, toml_version) {
            Ok(value) => DocumentTreeAndErrors {
                tree: crate::Value::OffsetDateTime(crate::OffsetDateTime { value, node: self }),
                errors: Vec::with_capacity(0),
            },
            Err(error) => DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![error],
            },
        }
    }
}

impl IntoDocumentTreeAndErrors<crate::Value> for ast::LocalDateTime {
    fn into_document_tree_and_errors(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> DocumentTreeAndErrors<crate::Value> {
        let range = self.range();
        let Some(_) = self.token() else {
            return DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::IncompleteNode { range }],
            };
        };

        match try_new_local_date_time(&self, toml_version) {
            Ok(value) => DocumentTreeAndErrors {
                tree: crate::Value::LocalDateTime(crate::LocalDateTime { value, node: self }),
                errors: Vec::with_capacity(0),
            },
            Err(error) => DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![error],
            },
        }
    }
}

impl IntoDocumentTreeAndErrors<crate::Value> for ast::LocalDate {
    fn into_document_tree_and_errors(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> DocumentTreeAndErrors<crate::Value> {
        let range = self.range();
        let Some(_) = self.token() else {
            return DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::IncompleteNode { range }],
            };
        };

        match try_new_local_date(&self, toml_version) {
            Ok(value) => DocumentTreeAndErrors {
                tree: crate::Value::LocalDate(crate::LocalDate { value, node: self }),
                errors: Vec::with_capacity(0),
            },
            Err(error) => DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![error],
            },
        }
    }
}

impl IntoDocumentTreeAndErrors<crate::Value> for ast::LocalTime {
    fn into_document_tree_and_errors(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> DocumentTreeAndErrors<crate::Value> {
        let range = self.range();
        let Some(_) = self.token() else {
            return DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![crate::Error::IncompleteNode { range }],
            };
        };

        match try_new_local_time(&self, toml_version) {
            Ok(value) => DocumentTreeAndErrors {
                tree: crate::Value::LocalTime(crate::LocalTime { value, node: self }),
                errors: Vec::with_capacity(0),
            },
            Err(error) => DocumentTreeAndErrors {
                tree: crate::Value::Incomplete { range },
                errors: vec![error],
            },
        }
    }
}
