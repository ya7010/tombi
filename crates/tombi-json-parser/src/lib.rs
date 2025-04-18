mod error;

pub use error::Error;
use indexmap::IndexMap;
use tombi_json_lexer::{lex, Lexed, Token};
use tombi_json_syntax::{SyntaxKind, T};
use tombi_json_tree::{ArrayNode, ObjectNode, Tree, ValueNode};
use tombi_json_value::Value;
use tombi_text::Range;

/// Parser for JSON documents
pub struct Parser<'a> {
    source: &'a str,
    lexed: Lexed,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let lexed = lex(source);
        Self {
            source,
            lexed,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.lexed.tokens.get(self.position)
    }

    fn peek_kind(&self) -> Option<SyntaxKind> {
        self.peek().map(|t| t.kind())
    }

    fn advance(&mut self) -> Option<&Token> {
        // Save the start position
        let position = self.position;
        let token = self.lexed.tokens.get(position);

        // Advance the position
        self.position += 1;

        // Skip trivia tokens
        while let Some(token) = self.lexed.tokens.get(self.position) {
            if token.kind().is_trivia() {
                self.position += 1;
            } else {
                break;
            }
        }

        token
    }

    fn expect(&mut self, kind: SyntaxKind) -> Result<&Token, crate::Error> {
        match self.peek() {
            Some(token) if token.kind() == kind => Ok(self.advance().unwrap()),
            Some(token) => Err(Error::UnexpectedToken {
                expected: kind,
                actual: token.kind(),
            }),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_string(&mut self) -> Result<String, crate::Error> {
        // Get the current token (without advancing the position)
        match self.peek() {
            Some(token) if token.kind() == SyntaxKind::STRING => {
                let span = token.span();
                // Get the string and advance the position
                let raw_str = &self.source[span.start().into()..span.end().into()];
                self.advance();

                // Remove the quotation marks
                let content = &raw_str[1..raw_str.len() - 1];

                // In a real implementation, we would process escape sequences here
                Ok(content.to_string())
            }
            Some(token) => Err(Error::UnexpectedToken {
                expected: SyntaxKind::STRING,
                actual: token.kind(),
            }),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_value(&mut self) -> Result<ValueNode, crate::Error> {
        match self.peek() {
            Some(token) => {
                match token.kind() {
                    SyntaxKind::STRING => {
                        let start_range = token.range();
                        let content = self.parse_string()?;
                        Ok(ValueNode::new(Value::String(content), start_range))
                    }
                    SyntaxKind::NUMBER => {
                        let token = self.peek().unwrap();
                        let span = token.span();
                        let range = token.range();
                        let num_str = &self.source[span.start().into()..span.end().into()];
                        self.advance();

                        // Parse as f64
                        match num_str.parse::<f64>() {
                            Ok(n) => Ok(ValueNode::new(Value::Number(n), range)),
                            Err(_) => Err(Error::InvalidValue),
                        }
                    }
                    SyntaxKind::NULL => {
                        let token = self.peek().unwrap();
                        let range = token.range();
                        self.advance();
                        Ok(ValueNode::new(Value::Null, range))
                    }
                    SyntaxKind::BOOLEAN => {
                        let token = self.peek().unwrap();
                        let span = token.span();
                        let range = token.range();
                        let bool_str = &self.source[span.start().into()..span.end().into()];
                        let value = bool_str == "true";
                        self.advance();

                        Ok(ValueNode::new(Value::Bool(value), range))
                    }
                    T!['['] => self.parse_array(),
                    T!['{'] => self.parse_object(),
                    _ => Err(Error::InvalidValue),
                }
            }
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_array(&mut self) -> Result<ValueNode, crate::Error> {
        // Consume the opening bracket
        let open_token = self.expect(T!['['])?;
        let start_range = open_token.range();
        let mut items = Vec::new();

        // Check if the array is empty
        if let Some(token) = self.peek() {
            if token.kind() == T![']'] {
                let close_token = self.advance().unwrap();
                let full_range = Range::new(start_range.start(), close_token.range().end());
                return Ok(ValueNode::new(Value::Array(Vec::new()), full_range));
            }
        }

        // Parse array elements
        loop {
            // Parse value
            let value = self.parse_value()?;
            items.push(value);

            // Check for comma or closing bracket
            match self.peek_kind() {
                Some(T![,]) => {
                    self.advance(); // Consume comma
                }
                Some(T![']']) => {
                    let close_token = self.advance().unwrap();
                    let full_range = Range::new(start_range.start(), close_token.range().end());

                    let array_node = ArrayNode {
                        items,
                        range: full_range,
                    };

                    return Ok(ValueNode::new(array_node.into(), full_range));
                }
                _ => {
                    return Err(Error::UnexpectedToken {
                        expected: T![']'],
                        actual: self.peek_kind().unwrap_or(SyntaxKind::EOF),
                    })
                }
            }

            // Check if we've reached the end of the array
            if let Some(token) = self.peek() {
                if token.kind() == T![']'] {
                    let close_token = self.advance().unwrap();
                    let full_range = Range::new(start_range.start(), close_token.range().end());

                    let array_node = ArrayNode {
                        items,
                        range: full_range,
                    };

                    return Ok(ValueNode::new(array_node.into(), full_range));
                }
            }
        }
    }

    fn parse_object(&mut self) -> Result<ValueNode, crate::Error> {
        // Consume the opening brace
        let open_token = self.expect(T!['{'])?;
        let start_range = open_token.range();
        let mut properties = IndexMap::new();
        let mut key_ranges = IndexMap::new();

        // Check if the object is empty
        if let Some(token) = self.peek() {
            if token.kind() == T!['}'] {
                let close_token = self.advance().unwrap();
                let full_range = Range::new(start_range.start(), close_token.range().end());
                return Ok(ValueNode::new(Value::Object(IndexMap::new()), full_range));
            }
        }

        // Parse object members
        loop {
            // Parse key (must be a string)
            if let Some(token) = self.peek() {
                if token.kind() != SyntaxKind::STRING {
                    return Err(Error::UnexpectedToken {
                        expected: SyntaxKind::STRING,
                        actual: token.kind(),
                    });
                }
            } else {
                return Err(Error::UnexpectedEof);
            }

            let key_range = self.peek().unwrap().range();
            let key = self.parse_string()?;

            // Check for duplicate keys
            if properties.contains_key(&key) {
                return Err(Error::DuplicateKey(key));
            }

            // Expect colon
            self.expect(T![:])?;

            // Parse value
            let value = self.parse_value()?;

            // Store key and value
            key_ranges.insert(key.clone(), key_range);
            properties.insert(key, value);

            // Check for comma or closing brace
            match self.peek_kind() {
                Some(T![,]) => {
                    self.advance(); // Consume comma
                }
                Some(T!['}']) => {
                    let close_token = self.advance().unwrap();
                    let full_range = Range::new(start_range.start(), close_token.range().end());

                    let object_node = ObjectNode {
                        properties,
                        key_ranges,
                        range: full_range,
                    };

                    return Ok(ValueNode::new(object_node.into(), full_range));
                }
                _ => {
                    return Err(Error::UnexpectedToken {
                        expected: T!['}'],
                        actual: self.peek_kind().unwrap_or(SyntaxKind::EOF),
                    })
                }
            }

            // Check if we've reached the end of the object
            if let Some(token) = self.peek() {
                if token.kind() == T!['}'] {
                    let close_token = self.advance().unwrap();
                    let full_range = Range::new(start_range.start(), close_token.range().end());

                    let object_node = ObjectNode {
                        properties,
                        key_ranges,
                        range: full_range,
                    };

                    return Ok(ValueNode::new(object_node.into(), full_range));
                }
            }
        }
    }

    fn parse_document(&mut self) -> Result<Tree, crate::Error> {
        // Skip leading trivia
        while let Some(token) = self.peek() {
            if token.kind().is_trivia() {
                self.advance();
            } else {
                break;
            }
        }

        let root = self.parse_value()?;

        // Skip trailing trivia
        while let Some(token) = self.peek() {
            if token.kind().is_trivia() {
                self.advance();
            } else {
                break;
            }
        }

        // Ensure all tokens have been consumed
        if let Some(token) = self.peek() {
            if token.kind() != SyntaxKind::EOF {
                return Err(Error::UnexpectedToken {
                    expected: SyntaxKind::EOF,
                    actual: token.kind(),
                });
            }
        }

        Ok(Tree::new(root))
    }
}

/// Parse a JSON string into a Tree
pub fn parse(source: &str) -> Result<Tree, crate::Error> {
    let mut parser = Parser::new(source);
    parser.parse_document()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        let source = "null";
        let tree = parse(source).unwrap();
        assert!(tree.root.is_null());
    }

    #[test]
    fn test_parse_boolean() {
        let source = "true";
        let tree = parse(source).unwrap();
        assert!(tree.root.is_bool());
        assert_eq!(tree.root.as_bool(), Some(true));

        let source = "false";
        let tree = parse(source).unwrap();
        assert!(tree.root.is_bool());
        assert_eq!(tree.root.as_bool(), Some(false));
    }

    #[test]
    fn test_parse_number() {
        let source = "42";
        let tree = parse(source).unwrap();
        assert!(tree.root.is_number());
        assert_eq!(tree.root.as_f64(), Some(42.0));

        let source = "-3.14";
        let tree = parse(source).unwrap();
        assert!(tree.root.is_number());
        assert_eq!(tree.root.as_f64(), Some(-3.14));
    }

    #[test]
    fn test_parse_string() {
        let source = r#""hello""#;
        let tree = parse(source).unwrap();
        assert!(tree.root.is_string());
        assert_eq!(tree.root.as_str(), Some("hello"));
    }

    #[test]
    fn test_parse_array() {
        let source = "[1, 2, 3]";
        let tree = parse(source).unwrap();
        assert!(tree.root.is_array());

        let source = "[]";
        let tree = parse(source).unwrap();
        assert!(tree.root.is_array());
    }

    #[test]
    fn test_parse_object() {
        let source = r#"{"a": 1, "b": 2}"#;
        let tree = parse(source).unwrap();
        assert!(tree.root.is_object());

        let source = "{}";
        let tree = parse(source).unwrap();
        assert!(tree.root.is_object());
    }

    #[test]
    fn test_parse_complex() {
        let source = r#"
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

        let tree = parse(source).unwrap();
        assert!(tree.root.is_object());
    }
}
