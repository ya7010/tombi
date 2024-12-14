pub mod string {
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
        value[3..value.len() - 3].to_string()
    }

    pub fn from_multi_line_literal_string(value: &str) -> String {
        value[3..value.len() - 3].to_string()
    }
}

pub mod integer {
    use std::num::ParseIntError;

    pub fn try_from_binary(value: &str) -> Result<i64, ParseIntError> {
        i64::from_str_radix(&value[2..].replace('_', ""), 2)
    }

    pub fn try_from_octal(value: &str) -> Result<i64, ParseIntError> {
        i64::from_str_radix(&value[2..].replace('_', ""), 8)
    }

    pub fn try_from_decimal(value: &str) -> Result<i64, ParseIntError> {
        i64::from_str_radix(&value.replace('_', ""), 10)
    }

    pub fn try_from_hexadecimal(value: &str) -> Result<i64, ParseIntError> {
        i64::from_str_radix(&value[2..].replace('_', ""), 16)
    }
}
