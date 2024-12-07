#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    value: f64,
    range: text::Range,
}

impl Float {
    pub(crate) fn try_new(
        text: &str,
        range: text::Range,
    ) -> Result<Self, std::num::ParseFloatError> {
        Ok(Self {
            value: text.parse()?,
            range,
        })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl TryFrom<ast::Float> for Float {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::Float) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Self::try_new(token.text(), token.text_range()).map_err(|err| {
            vec![crate::Error::ParseFloatError {
                error: err,
                range: token.text_range(),
            }]
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn inf() {
        let float = crate::value::Float::try_new("inf", text::Range::default()).unwrap();
        assert_eq!(float.value(), std::f64::INFINITY);
    }

    #[test]
    fn p_inf() {
        let float = crate::value::Float::try_new("+inf", text::Range::default()).unwrap();
        assert_eq!(float.value(), std::f64::INFINITY);
    }

    #[test]
    fn m_inf() {
        let float = crate::value::Float::try_new("-inf", text::Range::default()).unwrap();
        assert_eq!(float.value(), std::f64::NEG_INFINITY);
    }

    #[test]
    fn nan() {
        let float = crate::value::Float::try_new("nan", text::Range::default()).unwrap();
        assert!(float.value().is_nan());
    }

    #[test]
    fn p_nan() {
        let float = crate::value::Float::try_new("+nan", text::Range::default()).unwrap();
        assert!(float.value().is_nan());
    }

    #[test]
    fn m_nan() {
        let float = crate::value::Float::try_new("-nan", text::Range::default()).unwrap();
        assert!(float.value().is_nan());
    }
}
