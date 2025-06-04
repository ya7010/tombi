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

#[test]
fn parse_null() {
    let json = "null";
    let (value_arena, value_id) = parse(json);
    let value = value_arena.get(value_id.unwrap()).unwrap();
    match value {
        Value::Null => {}
        _ => panic!("not a null value"),
    }
}

#[test]
fn parse_array() {
    let json = "[1, 2, 3]";
    let (value_arena, value_id) = parse(json);
    let value = value_arena.get(value_id.unwrap()).unwrap();
    match value {
        Value::Array(array_id) => {
            let arr = value_arena.array_arena().get(*array_id).unwrap();
            let nums: Vec<f64> = arr
                .iter()
                .map(|vid| match value_arena.get(*vid).unwrap() {
                    Value::Number(n) => *n,
                    _ => panic!("not a number in array"),
                })
                .collect();
            assert_eq!(nums, vec![1.0, 2.0, 3.0]);
        }
        _ => panic!("not an array value"),
    }
}

#[test]
fn parse_object() {
    let json = r#"{"a": 1, "b": true}"#;
    let (value_arena, value_id) = parse(json);
    let value = value_arena.get(value_id.unwrap()).unwrap();
    match value {
        Value::Object(obj_id) => {
            let obj = value_arena.object_arena().get(*obj_id).unwrap();
            let a = obj
                .iter()
                .find(|(k, _)| value_arena.str_arena().get(**k) == Some("a"))
                .unwrap();
            let b = obj
                .iter()
                .find(|(k, _)| value_arena.str_arena().get(**k) == Some("b"))
                .unwrap();
            match value_arena.get(*a.1).unwrap() {
                Value::Number(n) => assert_eq!(*n, 1.0),
                _ => panic!("a is not a number"),
            }
            match value_arena.get(*b.1).unwrap() {
                Value::Bool(v) => assert_eq!(*v, true),
                _ => panic!("b is not a bool"),
            }
        }
        _ => panic!("not an object value"),
    }
}

#[test]
fn parse_nested() {
    let json = r#"{"arr": [null, {"x": 2}]}"#;
    let (value_arena, value_id) = parse(json);
    let value = value_arena.get(value_id.unwrap()).unwrap();
    match value {
        Value::Object(obj_id) => {
            let obj = value_arena.object_arena().get(*obj_id).unwrap();
            let arr_id = obj
                .iter()
                .find(|(k, _)| value_arena.str_arena().get(**k) == Some("arr"))
                .unwrap()
                .1;
            match value_arena.get(*arr_id).unwrap() {
                Value::Array(array_id) => {
                    let arr = value_arena.array_arena().get(*array_id).unwrap();
                    assert!(matches!(value_arena.get(arr[0]).unwrap(), Value::Null));
                    match value_arena.get(arr[1]).unwrap() {
                        Value::Object(inner_obj_id) => {
                            let inner_obj = value_arena.object_arena().get(*inner_obj_id).unwrap();
                            let x_id = inner_obj
                                .iter()
                                .find(|(k, _)| value_arena.str_arena().get(**k) == Some("x"))
                                .unwrap()
                                .1;
                            match value_arena.get(*x_id).unwrap() {
                                Value::Number(n) => assert_eq!(*n, 2.0),
                                _ => panic!("x is not a number"),
                            }
                        }
                        _ => panic!("not an object in array[1]"),
                    }
                }
                _ => panic!("arr is not an array"),
            }
        }
        _ => panic!("not an object value"),
    }
}
