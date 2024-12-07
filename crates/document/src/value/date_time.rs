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
    value: chrono::NaiveDateTime,
}

impl LocalDateTime {
    #[inline]
    pub fn value(&self) -> &chrono::NaiveDateTime {
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

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::test_serialize;

    test_serialize! {
        #[test]
        fn local_date_time(r#"ldt = 1979-05-27 07:32:00"#) -> Ok(json!({
            "ldt": "1979-05-27T07:32:00"
        }))
    }

    test_serialize! {
        #[test]
        fn local_date_time2(r#"ldt = 1979-05-27 07:32:00.9999"#) -> Ok(json!({
            "ldt": "1979-05-27T07:32:00.999900"
        }))
    }

    test_serialize! {
        #[test]
        fn local_date_time3(r#"ldt = 1979-05-27 07:32:00.99999999"#) -> Ok(json!({
            "ldt": "1979-05-27T07:32:00.999999990"
        }))
    }

    test_serialize! {
        #[test]
        fn invalid_date(r#"date = 0000-00-00"#) -> Err([
            ("invalid local date: input is out of range", ((0, 7), (0, 17)))
        ])
    }
}
