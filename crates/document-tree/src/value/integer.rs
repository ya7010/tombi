use crate::support::integer::{
    try_from_binary, try_from_decimal, try_from_hexadecimal, try_from_octal,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerKind {
    Binary(ast::IntegerBin),
    Decimal(ast::IntegerDec),
    Octal(ast::IntegerOct),
    Hexadecimal(ast::IntegerHex),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    kind: IntegerKind,
    value: i64,
}

impl Integer {
    #[inline]
    pub fn kind(&self) -> &IntegerKind {
        &self.kind
    }

    #[inline]
    pub fn value(&self) -> i64 {
        self.value
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        match self.kind() {
            IntegerKind::Binary(node) => node.token(),
            IntegerKind::Decimal(node) => node.token(),
            IntegerKind::Octal(node) => node.token(),
            IntegerKind::Hexadecimal(node) => node.token(),
        }
        .unwrap()
        .range()
    }

    #[inline]
    pub fn symbol_range(&self) -> text::Range {
        self.range()
    }
}

impl TryFrom<ast::IntegerBin> for Integer {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::IntegerBin) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        let range: text::Range = token.range();

        match try_from_binary(token.text()) {
            Ok(value) => Ok(Self {
                kind: IntegerKind::Binary(node),
                value,
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

        match try_from_octal(token.text()) {
            Ok(value) => Ok(Self {
                kind: IntegerKind::Octal(node),
                value,
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

        match try_from_decimal(token.text()) {
            Ok(value) => Ok(Self {
                kind: IntegerKind::Decimal(node),
                value,
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

        match try_from_hexadecimal(token.text()) {
            Ok(value) => Ok(Self {
                kind: IntegerKind::Hexadecimal(node),
                value,
            }),
            Err(error) => Err(vec![crate::Error::ParseIntError { error, range }]),
        }
    }
}
