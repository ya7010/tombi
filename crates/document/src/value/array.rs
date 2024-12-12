use crate::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrayKind {
    #[default]
    /// An array of tables.
    ///
    /// ```toml
    /// [[array]]
    /// ```
    ArrayOfTables,

    /// An array.
    ///
    /// ```toml
    /// key = [1, 2, 3]
    /// ```
    Array,
}

impl From<document_tree::ArrayKind> for ArrayKind {
    fn from(kind: document_tree::ArrayKind) -> Self {
        use document_tree::ArrayKind::*;

        match kind {
            ArrayOfTables | ParentArrayOfTables => Self::ArrayOfTables,
            Array => Self::Array,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Array {
    kind: ArrayKind,
    values: Vec<Value>,
}

impl Array {
    pub(crate) fn new(kind: ArrayKind) -> Self {
        Self {
            kind,
            values: Vec::new(),
        }
    }
    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn kind(&self) -> ArrayKind {
        self.kind
    }

    pub fn values(&self) -> &[Value] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<Value> {
        &mut self.values
    }
}

impl Into<Vec<Value>> for Array {
    fn into(self) -> Vec<Value> {
        self.values
    }
}

impl From<document_tree::Array> for Array {
    fn from(array: document_tree::Array) -> Self {
        Self {
            kind: array.kind().into(),
            values: Vec::<document_tree::Value>::from(array.values())
                .into_iter()
                .map(Value::from)
                .collect(),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Array {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.values.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Array {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let values = Vec::<Value>::deserialize(deserializer)?;
        Ok(Self {
            kind: ArrayKind::ArrayOfTables,
            values,
        })
    }
}
