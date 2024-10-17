use assert_matches::assert_matches;
use ast::AstNode;

#[test]

fn format_preformatted_text() {
    let source = r#"
key = "value"
bare_key = "value"
bare-key = "value"
1234 = "value"
"#
    .trim()
    .to_string();

    let result = formatter::format(&source);

    assert_matches!(result, Ok(_));
    assert_eq!(result.unwrap(), source);
}
