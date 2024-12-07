#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffsetDateTime {
    value: chrono::DateTime<chrono::FixedOffset>,
}

impl OffsetDateTime {
    #[inline]
    pub fn value(&self) -> &chrono::DateTime<chrono::FixedOffset> {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDateTime {
    value: chrono::DateTime<chrono::Local>,
}

impl LocalDateTime {
    #[inline]
    pub fn value(&self) -> &chrono::DateTime<chrono::Local> {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDate {
    value: chrono::NaiveDate,
}

impl LocalDate {
    #[inline]
    pub fn value(&self) -> &chrono::NaiveDate {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalTime {
    value: chrono::NaiveTime,
}

impl LocalTime {
    #[inline]
    pub fn value(&self) -> &chrono::NaiveTime {
        &self.value
    }
}

impl From<document_tree::OffsetDateTime> for OffsetDateTime {
    fn from(node: document_tree::OffsetDateTime) -> Self {
        Self {
            value: *node.value(),
        }
    }
}

impl From<document_tree::LocalDateTime> for LocalDateTime {
    fn from(node: document_tree::LocalDateTime) -> Self {
        Self {
            value: *node.value(),
        }
    }
}

impl From<document_tree::LocalDate> for LocalDate {
    fn from(node: document_tree::LocalDate) -> Self {
        Self {
            value: *node.value(),
        }
    }
}

impl From<document_tree::LocalTime> for LocalTime {
    fn from(node: document_tree::LocalTime) -> Self {
        Self {
            value: *node.value(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for OffsetDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for LocalDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for LocalDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for LocalTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}
