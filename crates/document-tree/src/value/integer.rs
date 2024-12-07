#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerKind {
    Binary,
    Decimal,
    Octal,
    Hexadecimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    kind: IntegerKind,
    value: isize,
    range: text::Range,
}

impl Integer {
    pub fn try_new_integer_bin(
        text: &str,
        range: text::Range,
    ) -> Result<Self, std::num::ParseIntError> {
        isize::from_str_radix(&text[2..], 2).map(|value| Self {
            kind: IntegerKind::Binary,
            value,
            range,
        })
    }

    pub fn try_new_integer_dec(
        text: &str,
        range: text::Range,
    ) -> Result<Self, std::num::ParseIntError> {
        isize::from_str_radix(text, 10).map(|value| Self {
            kind: IntegerKind::Decimal,
            value,
            range,
        })
    }

    pub fn try_new_integer_oct(
        text: &str,
        range: text::Range,
    ) -> Result<Self, std::num::ParseIntError> {
        isize::from_str_radix(&text[2..], 8).map(|value| Self {
            kind: IntegerKind::Octal,
            value,
            range,
        })
    }

    pub fn try_new_integer_hex(
        text: &str,
        range: text::Range,
    ) -> Result<Self, std::num::ParseIntError> {
        isize::from_str_radix(&text[2..], 16).map(|value| Self {
            kind: IntegerKind::Hexadecimal,
            value,
            range,
        })
    }

    #[inline]
    pub fn kind(&self) -> IntegerKind {
        self.kind
    }

    #[inline]
    pub fn value(&self) -> isize {
        self.value
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl TryFrom<ast::IntegerBin> for Integer {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::IntegerBin) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Self::try_new_integer_bin(token.text(), token.text_range()).map_err(|err| {
            vec![crate::Error::ParseIntError {
                error: err,
                range: token.text_range(),
            }]
        })
    }
}

impl TryFrom<ast::IntegerOct> for Integer {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::IntegerOct) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Self::try_new_integer_oct(token.text(), token.text_range()).map_err(|err| {
            vec![crate::Error::ParseIntError {
                error: err,
                range: token.text_range(),
            }]
        })
    }
}

impl TryFrom<ast::IntegerDec> for Integer {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::IntegerDec) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Self::try_new_integer_dec(token.text(), token.text_range()).map_err(|err| {
            vec![crate::Error::ParseIntError {
                error: err,
                range: token.text_range(),
            }]
        })
    }
}

impl TryFrom<ast::IntegerHex> for Integer {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::IntegerHex) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Self::try_new_integer_hex(token.text(), token.text_range()).map_err(|err| {
            vec![crate::Error::ParseIntError {
                error: err,
                range: token.text_range(),
            }]
        })
    }
}
