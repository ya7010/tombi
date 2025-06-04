use tombi_json_arena::{StrArena, Value, ValueArena};
use tombi_json_lexer::{tokenize, Token};

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
        .find(|t| format!("{:?}", t.kind()) == "STRING")
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
