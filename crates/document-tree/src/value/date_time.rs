use crate::{
    support::chrono::{
        try_new_local_date, try_new_local_date_time, try_new_local_time, try_new_offset_date_time,
    },
    DocumentTreeAndErrors, IntoDocumentTreeAndErrors, ValueImpl, ValueType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffsetDateTime {
    value: chrono::DateTime<chrono::FixedOffset>,
    node: ast::OffsetDateTime,
}

impl OffsetDateTime {
    #[inline]
    pub fn value(&self) -> &chrono::DateTime<chrono::FixedOffset> {
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
    // NOTE: `chrono::DateTime<chrono::Local>` is not enough to represent local date time.
    //       `chrono::Local.from_local_datetime(native_date_time)` cannot uniquely determine the time zone in some cases, so we handle NativeDateTime.
    value: chrono::NaiveDateTime,
    node: ast::LocalDateTime,
}

impl LocalDateTime {
    #[inline]
    pub fn value(&self) -> &chrono::NaiveDateTime {
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
    value: chrono::NaiveDate,
    node: ast::LocalDate,
}

impl LocalDate {
    #[inline]
    pub fn value(&self) -> &chrono::NaiveDate {
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
    value: chrono::NaiveTime,
    node: ast::LocalTime,
}

impl LocalTime {
    #[inline]
    pub fn value(&self) -> &chrono::NaiveTime {
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
