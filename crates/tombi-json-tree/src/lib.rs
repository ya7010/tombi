use tombi_json_value::{Map, Value};
use tombi_text::Range;

/// A struct representing a JSON tree with source position information
#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    /// The root node of the JSON tree
    pub root: ValueNode,
}

/// A JSON value with source code position information
#[derive(Debug, Clone, PartialEq)]
pub struct ValueNode {
    /// The JSON value
    pub value: Value,
    /// The position in the source code
    pub range: Range,
}

/// A JSON array with source code position information
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayNode {
    /// The array elements
    pub items: Vec<ValueNode>,
    /// The position of the entire array in the source code
    pub range: Range,
}

/// A JSON object with source code position information
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNode {
    /// The object properties
    pub properties: Map<String, ValueNode>,
    /// The position of the entire object in the source code
    pub range: Range,
}

impl ValueNode {
    /// Create a new JSON node
    pub fn new(value: Value, range: Range) -> Self {
        Self { value, range }
    }

    /// Check if the node is null
    pub fn is_null(&self) -> bool {
        self.value.is_null()
    }

    /// Check if the node is a boolean
    pub fn is_bool(&self) -> bool {
        self.value.is_bool()
    }

    /// Check if the node is a number
    pub fn is_number(&self) -> bool {
        self.value.is_number()
    }

    /// Check if the node is a string
    pub fn is_string(&self) -> bool {
        self.value.is_string()
    }

    /// Check if the node is an array
    pub fn is_array(&self) -> bool {
        self.value.is_array()
    }

    /// Check if the node is an object
    pub fn is_object(&self) -> bool {
        self.value.is_object()
    }

    /// Get as boolean value
    pub fn as_bool(&self) -> Option<bool> {
        self.value.as_bool()
    }

    /// Get as number value
    pub fn as_f64(&self) -> Option<f64> {
        self.value.as_f64()
    }

    /// Get as integer number value
    pub fn as_i64(&self) -> Option<i64> {
        self.value.as_i64()
    }

    /// Get as string reference
    pub fn as_str(&self) -> Option<&str> {
        self.value.as_str()
    }
}

impl From<ValueNode> for Value {
    fn from(node: ValueNode) -> Self {
        node.value
    }
}

impl From<&ValueNode> for Value {
    fn from(node: &ValueNode) -> Self {
        node.value.clone()
    }
}

impl From<ArrayNode> for Value {
    fn from(node: ArrayNode) -> Self {
        let values: Vec<Value> = node.items.into_iter().map(Into::into).collect();
        Value::Array(values)
    }
}

impl From<&ArrayNode> for Value {
    fn from(node: &ArrayNode) -> Self {
        let values: Vec<Value> = node.items.iter().map(Into::into).collect();
        Value::Array(values)
    }
}

impl From<ObjectNode> for Value {
    fn from(node: ObjectNode) -> Self {
        // Use IndexMap as an intermediate step
        let mut map = Map::new();
        for (key, value_node) in node.properties {
            map.insert(key, Value::from(value_node));
        }
        // Convert IndexMap to Value
        Value::Object(map)
    }
}

impl From<&ObjectNode> for Value {
    fn from(node: &ObjectNode) -> Self {
        // Use IndexMap as an intermediate step
        let mut map = Map::new();
        for (key, value_node) in &node.properties {
            map.insert(key.clone(), Value::from(value_node));
        }
        // Convert IndexMap to Value
        Value::Object(map)
    }
}

impl Tree {
    /// Create a new JSON tree
    pub fn new(root: ValueNode) -> Self {
        Self { root }
    }
}

impl From<Tree> for Value {
    fn from(tree: Tree) -> Self {
        tree.root.into()
    }
}

impl From<&Tree> for Value {
    fn from(tree: &Tree) -> Self {
        (&tree.root).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tombi_json_value::Number;
    use tombi_text::Position;

    #[test]
    fn test_json_node() {
        let pos = Position::new(1, 1);
        let range = Range::at(pos);

        let node = ValueNode::new(Value::Null, range);
        assert!(node.is_null());

        let bool_node = ValueNode::new(Value::Bool(true), range);
        assert!(bool_node.is_bool());
        assert_eq!(bool_node.as_bool(), Some(true));

        let number_node = ValueNode::new(Value::Number(Number::from_i64(42)), range);
        assert!(number_node.is_number());
        assert_eq!(number_node.as_i64(), Some(42));

        let string_node = ValueNode::new(Value::String("test".to_string()), range);
        assert!(string_node.is_string());
        assert_eq!(string_node.as_str(), Some("test"));
    }

    #[test]
    fn test_from_value_node() {
        let pos = Position::new(1, 1);
        let range = Range::at(pos);

        let string_value = Value::String("test".to_string());
        let node = ValueNode::new(string_value.clone(), range);

        let value: Value = node.into();
        assert_eq!(value, string_value);
    }

    #[test]
    fn test_from_array_node() {
        let pos = Position::new(1, 1);
        let range = Range::at(pos);

        let items = vec![
            ValueNode::new(Value::Number(Number::from_f64(1.0)), range),
            ValueNode::new(Value::Number(Number::from_f64(2.0)), range),
            ValueNode::new(Value::Number(Number::from_f64(3.0)), range),
        ];

        let array_node = ArrayNode { items, range };
        let value: Value = array_node.into();

        assert!(value.is_array());
        let array = value.as_array().unwrap();
        assert_eq!(array.len(), 3);
        assert_eq!(array[0], Value::Number(Number::from_f64(1.0)));
        assert_eq!(array[1], Value::Number(Number::from_f64(2.0)));
        assert_eq!(array[2], Value::Number(Number::from_f64(3.0)));
    }

    #[test]
    fn test_from_object_node() {
        let pos = Position::new(1, 1);
        let range = Range::at(pos);

        let mut properties = Map::new();
        properties.insert(
            "a".to_string(),
            ValueNode::new(Value::Number(Number::from_f64(1.0)), range),
        );
        properties.insert(
            "b".to_string(),
            ValueNode::new(Value::Number(Number::from_f64(2.0)), range),
        );

        let object_node = ObjectNode { properties, range };
        let value: Value = object_node.into();

        assert!(value.is_object());
        let obj = value.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.get("a").unwrap(), &Value::Number(Number::from_f64(1.0)));
        assert_eq!(obj.get("b").unwrap(), &Value::Number(Number::from_f64(2.0)));
    }
}
