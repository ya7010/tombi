pub fn from_bare_key(value: &str) -> String {
    value.to_string()
}

pub fn from_basic_string(value: &str) -> String {
    value[1..value.len() - 1].replace(r#"\""#, r#"""#)
}

pub fn from_literal_string(value: &str) -> String {
    value[1..value.len() - 1].replace(r#"\'"#, "'")
}

pub fn from_multi_line_basic_string(value: &str) -> String {
    value[3..value.len() - 3]
        .chars()
        .skip_while(|c| matches!(c, '\r' | '\n'))
        .collect()
}

pub fn from_multi_line_literal_string(value: &str) -> String {
    value[3..value.len() - 3]
        .chars()
        .skip_while(|c| matches!(c, '\r' | '\n'))
        .collect()
}
