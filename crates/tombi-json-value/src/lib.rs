use indexmap::IndexMap;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

/// Enum representing a JSON value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// `null` value
    Null,
    /// Boolean value
    Bool(bool),
    /// Number value (represented as a floating point)
    Number(f64),
    /// String value
    String(String),
    /// Array of values
    Array(Vec<Value>),
    /// Object (using IndexMap)
    Object(IndexMap<String, Value>),
}

impl Value {
    /// Check if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Check if the value is a boolean
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Check if the value is a number
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Check if the value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Check if the value is an array
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    /// Check if the value is an object
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// Get as boolean value
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Get as number value
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get as string reference
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get as array reference
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Get as mutable array reference
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Get as object reference
    pub fn as_object(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Get as mutable object reference
    pub fn as_object_mut(&mut self) -> Option<&mut IndexMap<String, Value>> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Number(f)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Number(i as f64)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_owned())
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(v: Vec<T>) -> Self {
        Value::Array(v.into_iter().map(Into::into).collect())
    }
}

impl<K, V> From<IndexMap<K, V>> for Value
where
    K: Into<String> + Hash + Eq,
    V: Into<Value>,
{
    fn from(m: IndexMap<K, V>) -> Self {
        Value::Object(m.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
    }
}

impl<K, V> From<HashMap<K, V>> for Value
where
    K: Into<String> + Hash + Eq,
    V: Into<Value>,
{
    fn from(m: HashMap<K, V>) -> Self {
        let mut index_map = IndexMap::with_capacity(m.len());
        for (k, v) in m {
            index_map.insert(k.into(), v.into());
        }
        Value::Object(index_map)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s.replace('"', "\\\"")),
            Value::Array(a) => {
                write!(f, "[")?;
                let mut first = true;
                for item in a {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                    first = false;
                }
                write!(f, "]")
            }
            Value::Object(o) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in o {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key.replace('"', "\\\""), value)?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => serializer.serialize_f64(*n),
            Value::String(s) => serializer.serialize_str(s),
            Value::Array(a) => a.serialize(serializer),
            Value::Object(o) => o.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(Value::Bool(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Value::Number(v as f64))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Value::Number(v as f64))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
                Ok(Value::Number(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::String(v.to_owned()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                Ok(Value::String(v))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Value::Null)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Value::Null)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }
                Ok(Value::Array(values))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut obj = IndexMap::new();
                while let Some((key, value)) = map.next_entry()? {
                    obj.insert(key, value);
                }
                Ok(Value::Object(obj))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}
