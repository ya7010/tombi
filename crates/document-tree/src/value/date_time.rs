#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffsetDateTime {
    value: chrono::DateTime<chrono::FixedOffset>,
    range: text::Range,
}

impl OffsetDateTime {
    pub(crate) fn try_new(text: &str, range: text::Range) -> Result<Self, chrono::ParseError> {
        Ok(Self {
            value: chrono::DateTime::parse_from_rfc3339(text)?,
            range,
        })
    }

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
    pub(crate) fn try_new(text: &str, range: text::Range) -> Result<Self, chrono::ParseError> {
        chrono::DateTime::parse_from_rfc3339(text).map(|value| Self {
            value: value.with_timezone(&chrono::Local),
            range,
        })
    }

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
    pub(crate) fn try_new(text: &str, range: text::Range) -> Result<Self, chrono::ParseError> {
        Ok(Self {
            value: chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d")?,
            range,
        })
    }

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
    pub(crate) fn try_new(text: &str, range: text::Range) -> Result<Self, chrono::ParseError> {
        Ok(Self {
            value: chrono::NaiveTime::parse_from_str(text, "%H:%M:%S%.f")?,
            range,
        })
    }

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
        Self::try_new(token.text(), token.text_range()).map_err(|err| {
            vec![crate::Error::ParseOffsetDateTimeError {
                error: err,
                range: token.text_range(),
            }]
        })
    }
}

impl TryFrom<ast::LocalDateTime> for LocalDateTime {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LocalDateTime) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Self::try_new(token.text(), token.text_range()).map_err(|err| {
            vec![crate::Error::ParseLocalDateTimeError {
                error: err,
                range: token.text_range(),
            }]
        })
    }
}

impl TryFrom<ast::LocalDate> for LocalDate {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LocalDate) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Self::try_new(token.text(), token.text_range()).map_err(|err| {
            vec![crate::Error::ParseLocalDateError {
                error: err,
                range: token.text_range(),
            }]
        })
    }
}

impl TryFrom<ast::LocalTime> for LocalTime {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::LocalTime) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Self::try_new(token.text(), token.text_range()).map_err(|err| {
            vec![crate::Error::ParseLocalTimeError {
                error: err,
                range: token.text_range(),
            }]
        })
    }
}
