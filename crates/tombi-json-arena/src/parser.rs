mod error;
use crate::parser::{Error as ParserError, ErrorKind as ParserErrorKind};
use crate::{StrId, Value, ValueArena, ValueId};
use ahash::{HashMap, HashMapExt};
pub use error::{Error, ErrorKind};
use tombi_json_lexer::Token;
use tombi_json_syntax::SyntaxKind;
use tombi_text::Range;

pub fn parse(json_text: &str) -> Result<(ValueId, ValueArena), Vec<crate::Error>> {
    let mut value_arena = ValueArena::default();
    let mut str_map = HashMap::new();
    let lexed = tombi_json_lexer::lex(json_text);
    if !lexed.errors.is_empty() {
        return Err(lexed.errors.into_iter().map(crate::Error::Lexer).collect());
    }
    let tokens = &lexed.tokens;
    let mut pos = 0;
    match parse_value(tokens, &mut pos, json_text, &mut value_arena, &mut str_map) {
        Ok(value_id) => Ok((value_id, value_arena)),
        Err(mut errs) => {
            if errs.is_empty() {
                errs.push(crate::Error::Parser(ParserError {
                    kind: ParserErrorKind::ExpectedToken,
                    range: Range::default(),
                }));
            }
            Err(errs)
        }
    }
}

fn parse_value<'a>(
    tokens: &[Token],
    pos: &mut usize,
    json_text: &'a str,
    value_arena: &mut ValueArena,
    str_map: &mut HashMap<&'a str, StrId>,
) -> Result<ValueId, Vec<crate::Error>> {
    while *pos < tokens.len() {
        let token = &tokens[*pos];
        match token.kind() {
            SyntaxKind::STRING => {
                *pos += 1;
                return Ok(parse_string(token, json_text, value_arena, str_map));
            }
            SyntaxKind::NUMBER => {
                *pos += 1;
                return parse_number(token, json_text, value_arena).ok_or_else(|| {
                    vec![crate::Error::Parser(ParserError {
                        kind: ParserErrorKind::UnexpectedToken,
                        range: token.range(),
                    })]
                });
            }
            SyntaxKind::BOOLEAN => {
                *pos += 1;
                return parse_bool(token, json_text, value_arena).ok_or_else(|| {
                    vec![crate::Error::Parser(ParserError {
                        kind: ParserErrorKind::UnexpectedToken,
                        range: token.range(),
                    })]
                });
            }
            SyntaxKind::NULL => {
                *pos += 1;
                return Ok(value_arena.alloc(Value::Null));
            }
            SyntaxKind::BRACKET_START => {
                return parse_array(tokens, pos, json_text, value_arena, str_map);
            }
            SyntaxKind::BRACE_START => {
                return parse_object(tokens, pos, json_text, value_arena, str_map);
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
    Err(vec![crate::Error::Parser(ParserError {
        kind: ParserErrorKind::ExpectedToken,
        range: Range::default(),
    })])
}

fn parse_string<'a>(
    token: &Token,
    json_text: &'a str,
    value_arena: &mut ValueArena,
    str_map: &mut HashMap<&'a str, StrId>,
) -> ValueId {
    let span = token.span();
    let value_str = &json_text[(usize::from(span.start()) + 1)..(usize::from(span.end()) - 1)];
    let str_id = str_map
        .entry(value_str)
        .or_insert_with(|| value_arena.str_arena_mut().alloc(value_str));
    value_arena.alloc(Value::String(*str_id))
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

fn parse_array<'a>(
    tokens: &[Token],
    pos: &mut usize,
    json_text: &'a str,
    value_arena: &mut ValueArena,
    str_map: &mut HashMap<&'a str, StrId>,
) -> Result<ValueId, Vec<crate::Error>> {
    *pos += 1; // skip [
    let mut elements = Vec::new();
    let mut errors = Vec::new();
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
            errors.push(crate::Error::Parser(ParserError {
                kind: ParserErrorKind::ExpectedToken,
                range: Range::default(),
            }));
            break;
        }
        match parse_value(tokens, pos, json_text, value_arena, str_map) {
            Ok(elem_id) => elements.push(elem_id),
            Err(mut es) => {
                errors.append(&mut es);
                break;
            }
        }
    }
    if errors.is_empty() {
        let array_id = value_arena.array_arena_mut().insert(elements);
        Ok(value_arena.alloc(Value::Array(array_id)))
    } else {
        Err(errors)
    }
}

fn parse_object<'a>(
    tokens: &[Token],
    pos: &mut usize,
    json_text: &'a str,
    value_arena: &mut ValueArena,
    str_map: &mut HashMap<&'a str, StrId>,
) -> Result<ValueId, Vec<crate::Error>> {
    *pos += 1; // skip {
    let mut map = HashMap::new();
    let mut errors = Vec::new();
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
            errors.push(crate::Error::Parser(ParserError {
                kind: ParserErrorKind::ExpectedToken,
                range: Range::default(),
            }));
            break;
        }
        let key_token = &tokens[*pos];
        if key_token.kind() != SyntaxKind::STRING {
            errors.push(crate::Error::Parser(ParserError {
                kind: ParserErrorKind::UnexpectedToken,
                range: key_token.range(),
            }));
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
            errors.push(crate::Error::Parser(ParserError {
                kind: ParserErrorKind::ExpectedToken,
                range: if *pos < tokens.len() {
                    tokens[*pos].range()
                } else {
                    Range::default()
                },
            }));
            break;
        }
        *pos += 1;
        while *pos < tokens.len() && tokens[*pos].kind().is_trivia() {
            *pos += 1;
        }
        match parse_value(tokens, pos, json_text, value_arena, str_map) {
            Ok(val_id) => {
                map.insert(key_id, val_id);
            }
            Err(mut es) => {
                errors.append(&mut es);
                break;
            }
        }
    }
    if errors.is_empty() {
        let obj_id = value_arena.object_arena_mut().insert(map);
        Ok(value_arena.alloc(Value::Object(obj_id)))
    } else {
        Err(errors)
    }
}
