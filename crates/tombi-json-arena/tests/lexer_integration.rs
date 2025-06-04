use tombi_json_arena::{StrArena, Value, ValueArena};
use tombi_json_lexer::{tokenize, Token};
use tombi_json_syntax::SyntaxKind;
#[test]
fn parse_simple_string() {
    let json = "\"hello\"";
    let mut str_arena = StrArena::default();
    let mut value_arena = ValueArena::default();

    let tokens: Vec<Token> = tokenize(json)
        .collect::<Result<_, _>>()
        .expect("tokenize failed");
    let string_token = tokens
        .iter()
        .find(|t| t.kind() == SyntaxKind::STRING)
        .expect("no string token");
    let span = string_token.span();
    let value_str = &json[span.start().into()..span.end().into()];
    let value_str = &value_str[1..value_str.len() - 1];
    let str_id = str_arena.alloc(value_str);
    let value_id = value_arena.alloc(Value::String(str_id));
    let value = value_arena.get(value_id).unwrap();
    match value {
        Value::String(sid) => {
            let s = str_arena.get(*sid).unwrap();
            assert_eq!(s, "hello");
        }
        _ => panic!("not a string value"),
    }
}

#[test]
fn parse_simple_number() {
    let json = "42";
    let mut value_arena = ValueArena::default();

    let tokens: Vec<Token> = tokenize(json)
        .collect::<Result<_, _>>()
        .expect("tokenize failed");

    let number_token = tokens
        .iter()
        .find(|t| t.kind() == SyntaxKind::NUMBER)
        .expect("no number token");
    let span = number_token.span();
    let value_str = &json[span.start().into()..span.end().into()];
    let num: f64 = value_str.parse().expect("parse number");
    let value_id = value_arena.alloc(Value::Number(num));
    let value = value_arena.get(value_id).unwrap();
    match value {
        Value::Number(n) => {
            assert_eq!(*n, 42.0);
        }
        _ => panic!("not a number value"),
    }
}

#[test]
fn parse_simple_boolean() {
    let json = "true";
    let mut value_arena = ValueArena::default();

    let tokens: Vec<Token> = tokenize(json)
        .collect::<Result<_, _>>()
        .expect("tokenize failed");
    let bool_token = tokens
        .iter()
        .find(|t| t.kind() == SyntaxKind::BOOLEAN)
        .expect("no boolean token");
    let span = bool_token.span();
    let value_str = &json[span.start().into()..span.end().into()];
    let b = match value_str {
        "true" => true,
        "false" => false,
        _ => panic!("unexpected boolean literal"),
    };
    let value_id = value_arena.alloc(Value::Bool(b));
    let value = value_arena.get(value_id).unwrap();
    match value {
        Value::Bool(v) => {
            assert_eq!(*v, true);
        }
        _ => panic!("not a boolean value"),
    }
}
