#[cfg(feature = "serde")]
pub(crate) struct DateTimeKey;

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for DateTimeKey {
    fn deserialize<D>(deserializer: D) -> Result<DateTimeKey, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> serde::de::Visitor<'de> for FieldVisitor {
            type Value = ();

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a valid date time field")
            }

            fn visit_str<E>(self, s: &str) -> Result<(), E>
            where
                E: serde::de::Error,
            {
                if s == crate::FIELD {
                    Ok(())
                } else {
                    Err(serde::de::Error::custom("expected field with custom name"))
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)?;
        Ok(DateTimeKey)
    }
}
