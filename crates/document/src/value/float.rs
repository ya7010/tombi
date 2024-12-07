#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    value: f64,
}

impl Float {
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
        assert_eq!(float.value(), std::f64::INFINITY);
    }

    #[test]
    fn p_inf() {
        let float = crate::value::Float::try_new("+inf").unwrap();
        assert_eq!(float.value(), std::f64::INFINITY);
    }

    #[test]
    fn m_inf() {
        let float = crate::value::Float::try_new("-inf").unwrap();
        assert_eq!(float.value(), std::f64::NEG_INFINITY);
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
