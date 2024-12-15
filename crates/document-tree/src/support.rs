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

pub mod float {
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    pub enum ParseError {
        #[error(transparent)]
        Std(#[from] std::num::ParseFloatError),
        #[error("Both sides of the underscore must be numbers.")]
        Underscore,
        #[error("Leading zeros are not allowed.")]
        LeadingZero,
    }

    pub fn try_from_float(value: &str) -> Result<f64, self::ParseError> {
        if value.chars().enumerate().any(|(i, c)| {
            if c == '_' {
                match (value.chars().nth(i - 1), value.chars().nth(i + 1)) {
                    (Some(digit1), Some(digit2)) => !digit1.is_digit(10) || !digit2.is_digit(10),
                    (None, _) | (_, None) => true,
                }
            } else {
                false
            }
        }) {
            return Err(self::ParseError::Underscore);
        }

        let int_slice = if value.contains('.') {
            value.split('.').next().unwrap()
        } else {
            value.split('e').next().unwrap()
        };

        let int_number = if int_slice.starts_with("+") || int_slice.starts_with("-") {
            &int_slice[1..]
        } else {
            int_slice
        };

        if int_number.len() > 1 && int_number.starts_with('0') {
            return Err(self::ParseError::LeadingZero);
        }

        Ok(value.replace("_", "").parse()?)
    }
}
