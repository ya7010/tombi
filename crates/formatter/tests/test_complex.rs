use assert_matches::assert_matches;
use config::{FormatOptions, TomlVersion};

#[test]
fn test_key_values() {
    let source = r#"
key = "value"
bare_key = "value"
bare-key = "value"
1234 = "value"
"#
    .trim_start()
    .to_string();

    let result = formatter::Formatter::new(TomlVersion::default(), &FormatOptions::default())
        .format(&source);

    assert_matches!(result, Ok(_));
    assert_eq!(result.unwrap(), source);
}
