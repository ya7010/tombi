use tombi_json_arena::{parse, Value};

#[test]
fn parse_simple_string() {
    let json = "\"hello\"";
    let (value_arena, value_id) = parse(json);
    let value = value_arena.get(value_id.unwrap()).unwrap();
    match value {
        Value::String(sid) => {
            let s = value_arena.str_arena().get(*sid).unwrap();
            assert_eq!(s, "hello");
        }
        _ => panic!("not a string value"),
    }
}

#[test]
fn parse_simple_number() {
    let json = "42";
    let (value_arena, value_id) = parse(json);
    let value = value_arena.get(value_id.unwrap()).unwrap();
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
    let (value_arena, value_id) = parse(json);
    let value = value_arena.get(value_id.unwrap()).unwrap();
    match value {
        Value::Bool(v) => {
            assert_eq!(*v, true);
        }
        _ => panic!("not a boolean value"),
    }
}
