pub use tombi_json_parser::{parse, Error as ParserError};
pub use tombi_json_tree::{ArrayNode, ObjectNode, Tree, ValueNode};
pub use tombi_json_value::{Map, Number, Value};

use serde::de::{
    self, DeserializeOwned, Deserializer as SerdeDeserializer, MapAccess, SeqAccess, Visitor,
};
use std::fmt;
use std::marker::PhantomData;

/// Error that can occur when deserializing JSON
#[derive(Debug)]
pub enum Error {
    /// JSON parsing error
    Parser(ParserError),
    /// Custom error message
    Custom(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parser(err) => write!(f, "JSON parser error: {}", err),
            Error::Custom(msg) => write!(f, "JSON deserialization error: {}", msg),
        }
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl From<ParserError> for Error {
    fn from(err: ParserError) -> Self {
        Error::Parser(err)
    }
}

/// Deserializer for JSON data
pub struct Deserializer {
    // Private fields can be added if needed
}

impl Deserializer {
    /// Creates a new JSON deserializer
    pub fn new() -> Self {
        Deserializer {}
    }

    /// Deserialize a JSON string into a Value
    pub fn from_str(s: &str) -> Result<Value, ParserError> {
        // Parse the JSON string into a Tree
        let tree = parse(s)?;

        // Convert the Tree to a Value
        Ok(tree.into())
    }
}

/// Deserialize an instance of type Value from a string of JSON text
pub fn from_str(s: &str) -> Result<Value, ParserError> {
    Deserializer::from_str(s)
}

// ValueDeserializer that implements serde::Deserializer
pub struct ValueDeserializer<'de> {
    value: Value,
    _marker: PhantomData<&'de ()>,
}

impl<'de> ValueDeserializer<'de> {
    pub fn new(value: Value) -> Self {
        ValueDeserializer {
            value,
            _marker: PhantomData,
        }
    }
}

impl<'de> SerdeDeserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Null => visitor.visit_unit(),
            Value::Bool(b) => visitor.visit_bool(b),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    visitor.visit_i64(i)
                } else if let Some(f) = n.as_f64() {
                    visitor.visit_f64(f)
                } else {
                    Err(Error::Custom("invalid number value".to_string()))
                }
            }
            Value::String(s) => visitor.visit_string(s),
            Value::Array(_) => self.deserialize_seq(visitor),
            Value::Object(_) => self.deserialize_map(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Bool(b) => visitor.visit_bool(b),
            _ => Err(Error::Custom(format!(
                "invalid type: expected bool, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Number(ref n) => {
                if let Some(i) = n.as_i64() {
                    visitor.visit_i64(i)
                } else if let Some(f) = n.as_f64() {
                    visitor.visit_i64(f as i64)
                } else {
                    Err(Error::Custom(format!(
                        "invalid type: expected i64, found {:?}",
                        self.value
                    )))
                }
            }
            _ => Err(Error::Custom(format!(
                "invalid type: expected i64, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Number(ref n) => {
                if let Some(i) = n.as_i64() {
                    if i >= 0 {
                        visitor.visit_u64(i as u64)
                    } else {
                        Err(Error::Custom(format!(
                            "invalid value: negative number for u64: {}",
                            i
                        )))
                    }
                } else if let Some(f) = n.as_f64() {
                    if f >= 0.0 {
                        visitor.visit_u64(f as u64)
                    } else {
                        Err(Error::Custom(format!(
                            "invalid value: negative number for u64: {}",
                            f
                        )))
                    }
                } else {
                    Err(Error::Custom(format!(
                        "invalid type: expected u64, found {:?}",
                        self.value
                    )))
                }
            }
            _ => Err(Error::Custom(format!(
                "invalid type: expected u64, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Number(ref n) => {
                if let Some(f) = n.as_f64() {
                    visitor.visit_f64(f)
                } else if let Some(i) = n.as_i64() {
                    visitor.visit_f64(i as f64)
                } else {
                    Err(Error::Custom(format!(
                        "invalid type: expected f64, found {:?}",
                        self.value
                    )))
                }
            }
            _ => Err(Error::Custom(format!(
                "invalid type: expected f64, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(ref s) => {
                let mut chars = s.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => visitor.visit_char(c),
                    _ => Err(Error::Custom(format!(
                        "invalid value: expected single character string, found {:?}",
                        self.value
                    ))),
                }
            }
            _ => Err(Error::Custom(format!(
                "invalid type: expected char, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(s) => visitor.visit_string(s),
            _ => Err(Error::Custom(format!(
                "invalid type: expected string, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Array(arr) => {
                let mut bytes = Vec::with_capacity(arr.len());
                for item in arr {
                    match item {
                        Value::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                if i >= 0 && i <= 255 {
                                    bytes.push(i as u8);
                                } else {
                                    return Err(Error::Custom(format!(
                                        "invalid value for byte: {}, should be 0-255",
                                        i
                                    )));
                                }
                            } else {
                                return Err(Error::Custom(format!(
                                    "invalid value for byte: {:?}",
                                    n
                                )));
                            }
                        }
                        _ => {
                            return Err(Error::Custom(format!(
                                "invalid type for byte array element: expected number, found {:?}",
                                item
                            )));
                        }
                    }
                }
                visitor.visit_bytes(&bytes)
            }
            Value::String(s) => visitor.visit_bytes(s.as_bytes()),
            _ => Err(Error::Custom(format!(
                "invalid type: expected array or string, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Null => visitor.visit_unit(),
            _ => Err(Error::Custom(format!(
                "invalid type: expected null, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Array(arr) => {
                let seq_access = ValueSeqAccess::new(arr);
                visitor.visit_seq(seq_access)
            }
            _ => Err(Error::Custom(format!(
                "invalid type: expected array, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Object(map) => {
                let map_access = ValueMapAccess::new(map);
                visitor.visit_map(map_access)
            }
            _ => Err(Error::Custom(format!(
                "invalid type: expected object, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(ref s) => visitor.visit_enum(ValueEnumAccess::new(s.clone(), None)),
            Value::Object(mut obj) if obj.len() == 1 => {
                let (variant, value) = obj
                    .into_iter()
                    .next()
                    .expect("Object with length 1 has no entries");
                visitor.visit_enum(ValueEnumAccess::new(variant, Some(value)))
            }
            _ => Err(Error::Custom(format!(
                "invalid type: expected string or map with single key, found {:?}",
                self.value
            ))),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

// 配列アクセス用のヘルパー構造体
struct ValueSeqAccess {
    elements: Vec<Value>,
    index: usize,
}

impl ValueSeqAccess {
    fn new(elements: Vec<Value>) -> Self {
        ValueSeqAccess { elements, index: 0 }
    }
}

impl<'de> SeqAccess<'de> for ValueSeqAccess {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.index >= self.elements.len() {
            return Ok(None);
        }

        let value = self.elements.remove(self.index);
        seed.deserialize(ValueDeserializer::new(value)).map(Some)
    }
}

// マップアクセス用のヘルパー構造体
struct ValueMapAccess {
    map: Map<String, Value>,
    keys: Vec<String>,
    index: usize,
}

impl ValueMapAccess {
    fn new(map: Map<String, Value>) -> Self {
        let keys: Vec<String> = map.keys().cloned().collect();
        ValueMapAccess {
            map,
            keys,
            index: 0,
        }
    }
}

impl<'de> MapAccess<'de> for ValueMapAccess {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.index >= self.keys.len() {
            return Ok(None);
        }

        let key = self.keys[self.index].clone();
        seed.deserialize(ValueDeserializer::new(Value::String(key)))
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key = &self.keys[self.index];
        self.index += 1;

        if let Some(value) = self.map.get(key) {
            let value = value.clone();
            seed.deserialize(ValueDeserializer::new(value))
        } else {
            Err(Error::Custom(format!("no value for key: {}", key)))
        }
    }
}

// 列挙型アクセス用のヘルパー構造体
struct ValueEnumAccess {
    variant: String,
    value: Option<Value>,
}

impl ValueEnumAccess {
    fn new(variant: String, value: Option<Value>) -> Self {
        ValueEnumAccess { variant, value }
    }
}

impl<'de> de::EnumAccess<'de> for ValueEnumAccess {
    type Error = Error;
    type Variant = ValueVariantAccess;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = self.variant.clone();
        let variant_deserializer = ValueDeserializer::new(Value::String(variant));
        let variant_value = seed.deserialize(variant_deserializer)?;
        let variant_access = ValueVariantAccess { value: self.value };

        Ok((variant_value, variant_access))
    }
}

// 列挙型バリアントアクセス用のヘルパー構造体
struct ValueVariantAccess {
    value: Option<Value>,
}

impl<'de> de::VariantAccess<'de> for ValueVariantAccess {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Some(_) => Err(Error::Custom(
                "invalid type: expected unit variant, found non-unit variant".to_string(),
            )),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(ValueDeserializer::new(value)),
            None => Err(Error::Custom(
                "invalid type: expected newtype variant, found unit variant".to_string(),
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Array(vec)) => {
                let seq_access = ValueSeqAccess::new(vec);
                visitor.visit_seq(seq_access)
            }
            Some(_) => Err(Error::Custom(
                "invalid type: expected tuple variant, found non-array".to_string(),
            )),
            None => Err(Error::Custom(
                "invalid type: expected tuple variant, found unit variant".to_string(),
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Object(map)) => {
                let map_access = ValueMapAccess::new(map);
                visitor.visit_map(map_access)
            }
            Some(_) => Err(Error::Custom(
                "invalid type: expected struct variant, found non-object".to_string(),
            )),
            None => Err(Error::Custom(
                "invalid type: expected struct variant, found unit variant".to_string(),
            )),
        }
    }
}

/// Deserialize an instance of type T from a string of JSON text
pub fn from_str_to<T>(s: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    // Parse the JSON string into a Value
    let value = Deserializer::from_str(s)?;

    // Deserialize the Value into type T directly
    from_value(value)
}

/// Deserialize an instance of type T from a Value
pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let deserializer = ValueDeserializer::new(value);
    T::deserialize(deserializer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn test_deserialize_null() {
        let json = "null";
        let value = from_str(json).unwrap();
        assert!(value.is_null());
    }

    #[test]
    fn test_deserialize_bool() {
        let json = "true";
        let value = from_str(json).unwrap();
        assert!(value.is_bool());
        assert_eq!(value.as_bool(), Some(true));

        let json = "false";
        let value = from_str(json).unwrap();
        assert!(value.is_bool());
        assert_eq!(value.as_bool(), Some(false));
    }

    #[test]
    fn test_deserialize_number() {
        let json = "42";
        let value = from_str(json).unwrap();
        assert!(value.is_number());
        assert_eq!(value.as_i64(), Some(42));

        let json = "-3.14";
        let value = from_str(json).unwrap();
        assert!(value.is_number());
        assert_eq!(value.as_f64(), Some(-3.14));
    }

    #[test]
    fn test_deserialize_string() {
        let json = r#""hello""#;
        let value = from_str(json).unwrap();
        assert!(value.is_string());
        assert_eq!(value.as_str(), Some("hello"));
    }

    #[test]
    fn test_deserialize_array() {
        let json = "[1, 2, 3]";
        let value = from_str(json).unwrap();
        assert!(value.is_array());
        assert_eq!(value.as_array().unwrap().len(), 3);

        let json = "[]";
        let value = from_str(json).unwrap();
        assert!(value.is_array());
        assert_eq!(value.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_deserialize_object() {
        let json = r#"{"a": 1, "b": 2}"#;
        let value = from_str(json).unwrap();
        assert!(value.is_object());
        assert_eq!(value.as_object().unwrap().len(), 2);

        let json = "{}";
        let value = from_str(json).unwrap();
        assert!(value.is_object());
        assert_eq!(value.as_object().unwrap().len(), 0);
    }

    #[test]
    fn test_deserialize_complex() {
        let json = r#"
        {
            "name": "John",
            "age": 30,
            "isStudent": false,
            "courses": ["Math", "Physics"],
            "address": {
                "city": "New York",
                "zip": "10001"
            }
        }
        "#;

        let value = from_str(json).unwrap();
        assert!(value.is_object());

        let obj = value.as_object().unwrap();
        assert_eq!(obj.get("name").unwrap().as_str(), Some("John"));
        assert_eq!(obj.get("age").unwrap().as_i64(), Some(30));
        assert_eq!(obj.get("isStudent").unwrap().as_bool(), Some(false));

        let courses = obj.get("courses").unwrap().as_array().unwrap();
        assert_eq!(courses.len(), 2);
        assert_eq!(courses[0].as_str(), Some("Math"));
        assert_eq!(courses[1].as_str(), Some("Physics"));

        let address = obj.get("address").unwrap().as_object().unwrap();
        assert_eq!(address.get("city").unwrap().as_str(), Some("New York"));
        assert_eq!(address.get("zip").unwrap().as_str(), Some("10001"));
    }

    #[test]
    fn test_deserialize_to_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Person {
            name: String,
            age: u8,
            is_student: bool,
        }

        let json = r#"{"name": "John", "age": 30, "is_student": false}"#;
        let person: Person = from_str_to(json).unwrap();

        assert_eq!(person.name, "John");
        assert_eq!(person.age, 30);
        assert_eq!(person.is_student, false);
    }

    #[test]
    fn test_deserialize_to_nested_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Address {
            city: String,
            zip: String,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct Person {
            name: String,
            age: u8,
            address: Address,
        }

        let json = r#"
        {
            "name": "John",
            "age": 30,
            "address": {
                "city": "New York",
                "zip": "10001"
            }
        }
        "#;

        let person: Person = from_str_to(json).unwrap();

        assert_eq!(person.name, "John");
        assert_eq!(person.age, 30);
        assert_eq!(person.address.city, "New York");
        assert_eq!(person.address.zip, "10001");
    }

    #[test]
    fn test_deserialize_to_enum() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Color {
            Red,
            Green,
            Blue,
            RGB(u8, u8, u8),
            HexCode(String),
        }

        let json = r#""Red""#;
        let color: Color = from_str_to(json).unwrap();
        assert_eq!(color, Color::Red);

        let json = r#"{"RGB": [255, 0, 0]}"#;
        let color: Color = from_str_to(json).unwrap();
        assert_eq!(color, Color::RGB(255, 0, 0));

        let json = r###"{"HexCode": "#FF0000"}"###;
        let color: Color = from_str_to(json).unwrap();
        assert_eq!(color, Color::HexCode("#FF0000".to_string()));
    }
}
