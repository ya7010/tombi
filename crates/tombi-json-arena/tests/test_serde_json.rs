use serde_json::json;

#[cfg(feature = "serde_json1")]
#[test]
fn test_parse_to_serde_json_value() {
    use tombi_json_arena::features::serde_json::to_value;
    use tombi_json_arena::parse;

    let cases = vec![
        ("\"hello\"", json!("hello")),
        ("42", json!(42.0)),
        ("true", json!(true)),
        ("null", json!(null)),
        ("[1, 2, 3]", json!([1.0, 2.0, 3.0])),
        ("{\"a\": 1, \"b\": true}", json!({"a": 1.0, "b": true})),
        (
            "{\"arr\": [null, {\"x\": 2}]}",
            json!({"arr": [null, {"x": 2.0}]}),
        ),
    ];
    for (src, expected) in cases {
        let (id, arena) = parse(src).expect("parse error");
        let actual = to_value(&id, &arena);
        assert_eq!(actual, expected, "failed for input: {}", src);
    }
}
