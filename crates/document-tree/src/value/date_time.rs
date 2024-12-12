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

impl TryFrom<ast::OffsetDateTime> for OffsetDateTime {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::OffsetDateTime) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.range();
        match chrono::DateTime::parse_from_rfc3339(token.text()) {
            Ok(value) => Ok(Self { value, node }),
            Err(error) => Err(vec![crate::Error::ParseOffsetDateTimeError {
                error,
                range,
            }]),
        }
    }
}

impl TryFrom<ast::LocalDateTime> for LocalDateTime {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LocalDateTime) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.range();
        let mut text = token.text().to_string();
        if text.chars().nth(10) == Some('T') {
            text.replace_range(10..11, " ");
        }

        match chrono::NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S%.f") {
            Ok(value) => Ok(Self { value, node }),
            Err(error) => Err(vec![crate::Error::ParseLocalDateTimeError { error, range }]),
        }
    }
}

impl TryFrom<ast::LocalDate> for LocalDate {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LocalDate) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.range();
        match chrono::NaiveDate::parse_from_str(token.text(), "%Y-%m-%d") {
            Ok(value) => Ok(Self { value, node }),
            Err(error) => Err(vec![crate::Error::ParseLocalDateError { error, range }]),
        }
    }
}

impl TryFrom<ast::LocalTime> for LocalTime {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LocalTime) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.range();
        match chrono::NaiveTime::parse_from_str(token.text(), "%H:%M:%S%.f") {
            Ok(value) => Ok(Self { value, node }),
            Err(error) => Err(vec![crate::Error::ParseLocalTimeError { error, range }]),
        }
    }
}
