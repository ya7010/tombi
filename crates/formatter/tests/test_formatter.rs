use assert_matches::assert_matches;

#[test]
fn format_text() {
    let result = formatter::format(
        r#"
key = "value"
bare_key = "value"
bare-key = "value"
1234 = "value"
"#,
    );

    let expected = r#"
key = "value"
bare_key = "value"
bare-key = "value"
1234 = "value"
"#
    .to_string();

    assert_matches!(result, Ok(_));
    assert_eq!(result.unwrap(), expected);
}
