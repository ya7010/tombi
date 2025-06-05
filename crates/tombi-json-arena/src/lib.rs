mod arena;
mod error;
use crate::error::Error;

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

use ahash::{HashMap, HashMapExt};
use tombi_json_lexer::Token;
use tombi_json_syntax::SyntaxKind;

/// JSON文字列をパースし、ValueArenaとValueIdを返す
pub fn parse(json_text: &str) -> Result<(ValueId, ValueArena), Vec<Error>> {
    let mut value_arena = ValueArena::default();
    let lexed = tombi_json_lexer::lex(json_text);
    if !lexed.errors.is_empty() {
        return Err(lexed.errors.into_iter().map(Error::Lexer).collect());
    }
    let tokens = &lexed.tokens;
    let mut pos = 0;
    let value_id = parse_value(tokens, &mut pos, json_text, &mut value_arena)
        .ok_or_else(|| vec![Error::Parse("No value found".to_string())])?;
    Ok((value_id, value_arena))
}

fn parse_value(
    tokens: &[Token],
    pos: &mut usize,
    json_text: &str,
    value_arena: &mut ValueArena,
) -> Option<ValueId> {
    while *pos < tokens.len() {
        let token = &tokens[*pos];
        match token.kind() {
            SyntaxKind::STRING => {
                *pos += 1;
                return Some(parse_string(token, json_text, value_arena));
            }
            SyntaxKind::NUMBER => {
                *pos += 1;
                return parse_number(token, json_text, value_arena);
            }
            SyntaxKind::BOOLEAN => {
                *pos += 1;
                return parse_bool(token, json_text, value_arena);
            }
            SyntaxKind::NULL => {
                *pos += 1;
                return Some(value_arena.alloc(Value::Null));
            }
            SyntaxKind::BRACKET_START => {
                return Some(parse_array(tokens, pos, json_text, value_arena));
            }
            SyntaxKind::BRACE_START => {
                return Some(parse_object(tokens, pos, json_text, value_arena));
            }
            SyntaxKind::COMMA
            | SyntaxKind::COLON
            | SyntaxKind::BRACKET_END
            | SyntaxKind::BRACE_END
            | SyntaxKind::WHITESPACE
            | SyntaxKind::LINE_BREAK
            | SyntaxKind::ROOT
            | SyntaxKind::ARRAY
            | SyntaxKind::OBJECT
            | SyntaxKind::MEMBER
            | SyntaxKind::VALUE
            | SyntaxKind::EOF
            | SyntaxKind::INVALID_TOKEN
            | SyntaxKind::TOMBSTONE
            | SyntaxKind::__LAST => {
                *pos += 1;
                continue;
            }
        }
    }
    None
}

fn parse_string(token: &Token, json_text: &str, value_arena: &mut ValueArena) -> ValueId {
    let span = token.span();
    let value_str = &json_text[span.start().into()..span.end().into()];
    let value_str = &value_str[1..value_str.len() - 1];
    let str_id = value_arena.str_arena_mut().alloc(value_str);
    value_arena.alloc(Value::String(str_id))
}

fn parse_number(token: &Token, json_text: &str, value_arena: &mut ValueArena) -> Option<ValueId> {
    let span = token.span();
    let value_str = &json_text[span.start().into()..span.end().into()];
    value_str
        .parse::<f64>()
        .ok()
        .map(|num| value_arena.alloc(Value::Number(num)))
}

fn parse_bool(token: &Token, json_text: &str, value_arena: &mut ValueArena) -> Option<ValueId> {
    let span = token.span();
    let value_str = &json_text[span.start().into()..span.end().into()];
    match value_str {
        "true" => Some(value_arena.alloc(Value::Bool(true))),
        "false" => Some(value_arena.alloc(Value::Bool(false))),
        _ => None,
    }
}

fn parse_array(
    tokens: &[Token],
    pos: &mut usize,
    json_text: &str,
    value_arena: &mut ValueArena,
) -> ValueId {
    *pos += 1; // skip [
    let mut elements = Vec::new();
    loop {
        while *pos < tokens.len()
            && (tokens[*pos].kind() == SyntaxKind::COMMA || tokens[*pos].kind().is_trivia())
        {
            *pos += 1;
        }
        if *pos < tokens.len() && tokens[*pos].kind() == SyntaxKind::BRACKET_END {
            *pos += 1;
            break;
        }
        if *pos >= tokens.len() {
            break;
        }
        if let Some(elem_id) = parse_value(tokens, pos, json_text, value_arena) {
            elements.push(elem_id);
        } else {
            break;
        }
    }
    let array_id = value_arena.array_arena_mut().insert(elements);
    value_arena.alloc(Value::Array(array_id))
}

fn parse_object(
    tokens: &[Token],
    pos: &mut usize,
    json_text: &str,
    value_arena: &mut ValueArena,
) -> ValueId {
    *pos += 1; // skip {
    let mut map = HashMap::new();
    loop {
        while *pos < tokens.len()
            && (tokens[*pos].kind().is_trivia() || tokens[*pos].kind() == SyntaxKind::COMMA)
        {
            *pos += 1;
        }
        if *pos < tokens.len() && tokens[*pos].kind() == SyntaxKind::BRACE_END {
            *pos += 1;
            break;
        }
        if *pos >= tokens.len() {
            break;
        }
        let key_token = &tokens[*pos];
        if key_token.kind() != SyntaxKind::STRING {
            break;
        }
        let key_span = key_token.span();
        let key_str = &json_text[key_span.start().into()..key_span.end().into()];
        let key_str = &key_str[1..key_str.len() - 1];
        let key_id = value_arena.str_arena_mut().alloc(key_str);
        *pos += 1;
        while *pos < tokens.len() && tokens[*pos].kind().is_trivia() {
            *pos += 1;
        }
        if *pos >= tokens.len() || tokens[*pos].kind() != SyntaxKind::COLON {
            break;
        }
        *pos += 1;
        while *pos < tokens.len() && tokens[*pos].kind().is_trivia() {
            *pos += 1;
        }
        if let Some(val_id) = parse_value(tokens, pos, json_text, value_arena) {
            map.insert(key_id, val_id);
        } else {
            break;
        }
    }
    let obj_id = value_arena.object_arena_mut().insert(map);
    value_arena.alloc(Value::Object(obj_id))
}
