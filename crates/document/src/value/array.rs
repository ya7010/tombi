use itertools::Itertools;

use crate::{IntoDocument, TableKind, ToTomlString, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrayKind {
    #[default]
    /// An array of tables.
    ///
    /// ```toml
    /// [[array]]
    /// ```
    ArrayOfTable,

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
            ArrayOfTable | ParentArrayOfTable => Self::ArrayOfTable,
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
    pub fn new(kind: ArrayKind) -> Self {
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

impl From<Array> for Vec<Value> {
    fn from(val: Array) -> Self {
        val.values
    }
}

impl IntoDocument<Array> for document_tree::Array {
    fn into_document(self, toml_version: toml_version::TomlVersion) -> Array {
        Array {
            kind: self.kind().into(),
            values: Vec::<document_tree::Value>::from(self.values())
                .into_iter()
                .map(|value| value.into_document(toml_version))
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
            kind: ArrayKind::ArrayOfTable,
            values,
        })
    }
}

impl ToTomlString for Array {
    fn to_toml_string(&self, result: &mut std::string::String, parent_keys: &[&crate::Key]) {
        match self.kind {
            ArrayKind::Array => {
                result.push('[');
                if !self.values.is_empty() {
                    for (i, value) in self.values.iter().enumerate() {
                        if i != 0 {
                            result.push_str(", ");
                        }
                        value.to_toml_string(result, parent_keys);
                    }
                }
                result.push(']');
            }
            ArrayKind::ArrayOfTable => {
                for value in self.values.iter() {
                    result.push_str(&format!(
                        "[[{}]]\n",
                        parent_keys
                            .iter()
                            .map(ToString::to_string)
                            .collect_vec()
                            .join(".")
                    ));
                    if let Value::Table(table) = value {
                        for (key, value) in table.key_values() {
                            match value {
                                Value::Table(table) if table.kind() == TableKind::KeyValue => {
                                    table.to_toml_string(
                                        result,
                                        &parent_keys
                                            .iter()
                                            .chain(&[key])
                                            .map(|key| *key)
                                            .collect_vec(),
                                    );
                                }
                                _ => {
                                    result.push_str(&format!("{} = ", key));
                                    value.to_toml_string(
                                        result,
                                        &parent_keys
                                            .iter()
                                            .chain(&[key])
                                            .map(|key| *key)
                                            .collect_vec(),
                                    );
                                }
                            }
                            result.push('\n');
                        }
                    }
                }
            }
        }
    }
}
