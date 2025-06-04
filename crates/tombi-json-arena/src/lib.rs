mod arena;
pub use arena::{ArrayArena, ArrayId, ObjectArena, ObjectId, StrArena, StrId, ValueArena, ValueId};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(StrId),
    Array(ArrayId),
    Object(ObjectId),
}

use tombi_json_lexer::{lex, Token};
use tombi_json_syntax::SyntaxKind;

/// JSON文字列をパースし、ValueArenaとValueIdを返す
pub fn parse(json_text: &str) -> (StrArena, ValueArena, Option<ValueId>) {
    let mut str_arena = StrArena::default();
    let mut value_arena = ValueArena::default();
    let tokens: Vec<Token> = lex(json_text).tokens;
    let mut value_id = None;
    for token in tokens.iter() {
        match token.kind() {
            SyntaxKind::STRING => {
                let span = token.span();
                let value_str = &json_text[span.start().into()..span.end().into()];
                let value_str = &value_str[1..value_str.len() - 1];
                let str_id = str_arena.alloc(value_str);
                value_id = Some(value_arena.alloc(Value::String(str_id)));
            }
            SyntaxKind::NUMBER => {
                let span = token.span();
                let value_str = &json_text[span.start().into()..span.end().into()];
                if let Ok(num) = value_str.parse::<f64>() {
                    value_id = Some(value_arena.alloc(Value::Number(num)));
                }
            }
            SyntaxKind::BOOLEAN => {
                let span = token.span();
                let value_str = &json_text[span.start().into()..span.end().into()];
                let b = match value_str {
                    "true" => true,
                    "false" => false,
                    _ => continue,
                };
                value_id = Some(value_arena.alloc(Value::Bool(b)));
            }
            SyntaxKind::NULL => {
                value_id = Some(value_arena.alloc(Value::Null));
            }
            _ => {}
        }
    }
    (str_arena, value_arena, value_id)
}
