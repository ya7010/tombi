use crate::ToTomlString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerKind {
    Binary,
    Decimal,
    Octal,
    Hexadecimal,
}

impl From<document_tree::IntegerKind> for IntegerKind {
    fn from(kind: document_tree::IntegerKind) -> Self {
        match kind {
            document_tree::IntegerKind::Binary(_) => Self::Binary,
            document_tree::IntegerKind::Decimal(_) => Self::Decimal,
            document_tree::IntegerKind::Octal(_) => Self::Octal,
            document_tree::IntegerKind::Hexadecimal(_) => Self::Hexadecimal,
        }
    }
}

impl From<&document_tree::IntegerKind> for IntegerKind {
    fn from(kind: &document_tree::IntegerKind) -> Self {
        match kind {
            document_tree::IntegerKind::Binary(_) => Self::Binary,
            document_tree::IntegerKind::Decimal(_) => Self::Decimal,
            document_tree::IntegerKind::Octal(_) => Self::Octal,
            document_tree::IntegerKind::Hexadecimal(_) => Self::Hexadecimal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    kind: IntegerKind,
    value: i64,
}

impl Integer {
    #[inline]
    pub fn new(value: i64) -> Self {
        Self {
            kind: IntegerKind::Decimal,
            value,
        }
    }

    #[inline]
    pub fn kind(&self) -> IntegerKind {
        self.kind
    }

    #[inline]
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl From<document_tree::Integer> for Integer {
    fn from(node: document_tree::Integer) -> Self {
        Self {
            kind: node.kind().into(),
            value: node.value(),
        }
    }
}

impl ToTomlString for Integer {
    fn to_toml_string(&self, result: &mut std::string::String, _parent_keys: &[&crate::Key]) {
        match self.kind {
            IntegerKind::Binary => {
                result.push_str("0b");
                result.push_str(&format!("{:b}", self.value));
            }
            IntegerKind::Decimal => {
                result.push_str(&self.value.to_string());
            }
            IntegerKind::Octal => {
                result.push_str("0o");
                result.push_str(&format!("{:o}", self.value));
            }
            IntegerKind::Hexadecimal => {
                result.push_str("0x");
                result.push_str(&format!("{:x}", self.value));
            }
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        Ok(Self {
            kind: IntegerKind::Decimal,
            value,
        })
    }
}
