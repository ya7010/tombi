#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("invalid escape sequence")]
    InvalidEscapeSequence,

    #[error("invalid Unicode escape sequence")]
    InvalidUnicodeEscapeSequence,

    #[error("invalid Unicode code point")]
    InvalidUnicodeCodePoint,

    #[error("invalid newline character in input")]
    InvalidNewline,

    #[error("invalid control character in input")]
    InvalidControlCharacter,

    #[error("trailing backslash in input")]
    TrailingBackslash,
}

pub fn from_bare_key(value: &str) -> String {
    value.to_string()
}

pub fn try_from_basic_string(value: &str) -> Result<String, ParseError> {
    escape_basic_string(&value[1..value.len() - 1])
}

pub fn try_from_literal_string(value: &str) -> Result<String, ParseError> {
    // NOTE: Literal strings are not escaped.
    Ok(value[1..value.len() - 1].to_string())
}

pub fn try_from_multi_line_basic_string(value: &str) -> Result<String, ParseError> {
    escape_basic_string(
        &value[3..value.len() - 3]
            .chars()
            .skip_while(|c| matches!(c, '\r' | '\n'))
            .collect::<String>(),
    )
}

pub fn try_from_multi_line_literal_string(value: &str) -> Result<String, ParseError> {
    Ok(value[3..value.len() - 3]
        .chars()
        .skip_while(|c| matches!(c, '\r' | '\n'))
        .collect())
}

fn escape_basic_string(input: &str) -> Result<String, ParseError> {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut unicode_buf = String::new();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(&next_c) = chars.peek() {
                match next_c {
                    'b' => {
                        output.push('\u{0008}');
                        chars.next();
                    }
                    't' => {
                        output.push('\t');
                        chars.next();
                    }
                    'n' => {
                        output.push('\n');
                        chars.next();
                    }
                    'f' => {
                        output.push('\u{000C}');
                        chars.next();
                    }
                    'r' => {
                        output.push('\r');
                        chars.next();
                    }
                    '"' => {
                        output.push('\"');
                        chars.next();
                    }
                    '\\' => {
                        output.push('\\');
                        chars.next();
                    }
                    'u' => {
                        chars.next(); // consume 'u'
                        unicode_buf.clear();
                        for _ in 0..4 {
                            if let Some(hex_digit) = chars.next() {
                                unicode_buf.push(hex_digit);
                            } else {
                                return Err(ParseError::InvalidUnicodeEscapeSequence);
                            }
                        }
                        if let Ok(code_point) = u32::from_str_radix(&unicode_buf, 16) {
                            if let Some(unicode_char) = std::char::from_u32(code_point) {
                                output.push(unicode_char);
                            } else {
                                return Err(ParseError::InvalidUnicodeCodePoint);
                            }
                        } else {
                            return Err(ParseError::InvalidUnicodeEscapeSequence);
                        }
                    }
                    'U' => {
                        chars.next(); // consume 'U'
                        unicode_buf.clear();
                        for _ in 0..8 {
                            if let Some(hex_digit) = chars.next() {
                                unicode_buf.push(hex_digit);
                            } else {
                                return Err(ParseError::InvalidUnicodeEscapeSequence);
                            }
                        }
                        if let Ok(code_point) = u32::from_str_radix(&unicode_buf, 16) {
                            if let Some(unicode_char) = std::char::from_u32(code_point) {
                                output.push(unicode_char);
                            } else {
                                return Err(ParseError::InvalidUnicodeCodePoint);
                            }
                        } else {
                            return Err(ParseError::InvalidUnicodeEscapeSequence);
                        }
                    }
                    '\n' => {
                        // Skip newline characters
                        chars.next();
                        while let Some(&c) = chars.peek() {
                            if c.is_whitespace() {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                    '\r' => {
                        // Skip newline characters
                        chars.next();
                        if let Some(&'\n') = chars.peek() {
                            chars.next();
                        } else {
                            return Err(ParseError::InvalidNewline);
                        }
                        while let Some(&c) = chars.peek() {
                            if c.is_whitespace() {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                    _ => {
                        return Err(ParseError::InvalidEscapeSequence);
                    }
                }
            } else {
                return Err(ParseError::TrailingBackslash);
            }
        } else if matches!(c, '\u{0000}'..='\u{0008}' | '\u{000B}'..='\u{001F}' | '\u{007F}') {
            return Err(ParseError::InvalidControlCharacter);
        } else {
            output.push(c);
        }
    }
    Ok(output)
}
