use crate::{
    support::chrono::{
        try_new_local_date, try_new_local_date_time, try_new_local_time, try_new_offset_date_time,
    },
    TryIntoDocumentTree,
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

impl TryIntoDocumentTree<OffsetDateTime> for ast::OffsetDateTime {
    fn try_into_document_tree(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> Result<OffsetDateTime, Vec<crate::Error>> {
        match try_new_offset_date_time(&self, toml_version) {
            Ok(value) => Ok(OffsetDateTime { value, node: self }),
            Err(error) => Err(vec![error]),
        }
    }
}

impl TryIntoDocumentTree<LocalDateTime> for ast::LocalDateTime {
    fn try_into_document_tree(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> Result<LocalDateTime, Vec<crate::Error>> {
        match try_new_local_date_time(&self, toml_version) {
            Ok(value) => Ok(LocalDateTime { value, node: self }),
            Err(error) => Err(vec![error]),
        }
    }
}

impl TryIntoDocumentTree<LocalDate> for ast::LocalDate {
    fn try_into_document_tree(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> Result<LocalDate, Vec<crate::Error>> {
        match try_new_local_date(&self, toml_version) {
            Ok(value) => Ok(LocalDate { value, node: self }),
            Err(error) => Err(vec![error]),
        }
    }
}

impl TryIntoDocumentTree<LocalTime> for ast::LocalTime {
    fn try_into_document_tree(
        self,
        toml_version: toml_version::TomlVersion,
    ) -> Result<LocalTime, Vec<crate::Error>> {
        match try_new_local_time(&self, toml_version) {
            Ok(value) => Ok(LocalTime { value, node: self }),
            Err(error) => Err(vec![error]),
        }
    }
}
