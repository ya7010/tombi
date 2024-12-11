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
    symbol_range: text::Range,
}

impl Integer {
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

    #[inline]
    pub fn symbol_range(&self) -> text::Range {
        self.symbol_range
    }
}

impl TryFrom<ast::IntegerBin> for Integer {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::IntegerBin) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range: text::Range = token.range();

        match isize::from_str_radix(&token.text()[2..], 2) {
            Ok(value) => Ok(Self {
                kind: IntegerKind::Binary,
                value,
                range,
                symbol_range: range,
            }),
            Err(error) => Err(vec![crate::Error::ParseIntError { error, range }]),
        }
    }
}

impl TryFrom<ast::IntegerOct> for Integer {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::IntegerOct) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.range();

        match isize::from_str_radix(&token.text()[2..], 8) {
            Ok(value) => Ok(Self {
                kind: IntegerKind::Octal,
                value,
                range,
                symbol_range: range,
            }),
            Err(error) => Err(vec![crate::Error::ParseIntError { error, range }]),
        }
    }
}

impl TryFrom<ast::IntegerDec> for Integer {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::IntegerDec) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.range();

        match isize::from_str_radix(token.text(), 10) {
            Ok(value) => Ok(Self {
                kind: IntegerKind::Decimal,
                value,
                range,
                symbol_range: range,
            }),
            Err(error) => Err(vec![crate::Error::ParseIntError { error, range }]),
        }
    }
}

impl TryFrom<ast::IntegerHex> for Integer {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::IntegerHex) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range = token.range();

        match isize::from_str_radix(&token.text()[2..], 16) {
            Ok(value) => Ok(Self {
                kind: IntegerKind::Hexadecimal,
                value,
                range,
                symbol_range: range,
            }),
            Err(error) => Err(vec![crate::Error::ParseIntError { error, range }]),
        }
    }
}
