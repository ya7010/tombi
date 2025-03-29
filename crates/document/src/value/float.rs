use super::ToTomlString;

#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    value: f64,
}

impl Float {
    #[inline]
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl From<document_tree::Float> for Float {
    fn from(node: document_tree::Float) -> Self {
        Self {
            value: node.value(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Float {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Float {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        f64::deserialize(deserializer).map(|value| Self { value })
    }
}

impl ToTomlString for Float {
    fn to_toml_string(&self, result: &mut std::string::String, _indent: usize) {
        if self.value.is_infinite() {
            if self.value.is_sign_positive() {
                result.push_str("inf");
            } else {
                result.push_str("-inf");
            }
        } else if self.value.is_nan() {
            result.push_str("nan");
        } else {
            result.push_str(&self.value.to_string());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Float {
        fn try_new(value: &str) -> Result<Self, Box<dyn std::error::Error>> {
            Ok(Self {
                value: value.parse()?,
            })
        }
    }

    #[test]
    fn inf() {
        let float = crate::value::Float::try_new("inf").unwrap();
        assert_eq!(float.value(), f64::INFINITY);
    }

    #[test]
    fn p_inf() {
        let float = crate::value::Float::try_new("+inf").unwrap();
        assert_eq!(float.value(), f64::INFINITY);
    }

    #[test]
    fn m_inf() {
        let float = crate::value::Float::try_new("-inf").unwrap();
        assert_eq!(float.value(), f64::NEG_INFINITY);
    }

    #[test]
    fn nan() {
        let float = crate::value::Float::try_new("nan").unwrap();
        assert!(float.value().is_nan());
    }

    #[test]
    fn p_nan() {
        let float = crate::value::Float::try_new("+nan").unwrap();
        assert!(float.value().is_nan());
    }

    #[test]
    fn m_nan() {
        let float = crate::value::Float::try_new("-nan").unwrap();
        assert!(float.value().is_nan());
    }
}
