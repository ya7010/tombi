#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffsetDateTime {
    value: chrono::DateTime<chrono::FixedOffset>,
    range: text::Range,
}

impl OffsetDateTime {
    #[inline]
    pub fn value(&self) -> &chrono::DateTime<chrono::FixedOffset> {
        &self.value
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDateTime {
    value: chrono::DateTime<chrono::Local>,
    range: text::Range,
}

impl LocalDateTime {
    #[inline]
    pub fn value(&self) -> &chrono::DateTime<chrono::Local> {
        &self.value
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDate {
    value: chrono::NaiveDate,
    range: text::Range,
}

impl LocalDate {
    #[inline]
    pub fn value(&self) -> &chrono::NaiveDate {
        &self.value
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalTime {
    value: chrono::NaiveTime,
    range: text::Range,
}

impl LocalTime {
    #[inline]
    pub fn value(&self) -> &chrono::NaiveTime {
        &self.value
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl TryFrom<ast::OffsetDateTime> for OffsetDateTime {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::OffsetDateTime) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.text_range();
        match chrono::DateTime::parse_from_rfc3339(token.text()) {
            Ok(value) => Ok(Self { value, range }),
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
        let range = token.text_range();
        match chrono::DateTime::parse_from_rfc3339(token.text()) {
            Ok(value) => Ok(Self {
                value: value.with_timezone(&chrono::Local),
                range,
            }),
            Err(error) => Err(vec![crate::Error::ParseLocalDateTimeError { error, range }]),
        }
    }
}

impl TryFrom<ast::LocalDate> for LocalDate {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LocalDate) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.text_range();
        match chrono::NaiveDate::parse_from_str(token.text(), "%Y-%m-%d") {
            Ok(value) => Ok(Self { value, range }),
            Err(error) => Err(vec![crate::Error::ParseLocalDateError { error, range }]),
        }
    }
}

impl TryFrom<ast::LocalTime> for LocalTime {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LocalTime) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.text_range();
        match chrono::NaiveTime::parse_from_str(token.text(), "%H:%M:%S%.f") {
            Ok(value) => Ok(Self { value, range }),
            Err(error) => Err(vec![crate::Error::ParseLocalTimeError { error, range }]),
        }
    }
}
