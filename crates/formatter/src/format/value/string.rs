use super::LiteralNode;
use crate::format::Format;
use ast::AstNode;
use config::QuoteStyle;
use std::fmt::Write;

impl Format for ast::BasicString {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for comment in self.leading_comments() {
            comment.fmt(f)?;
        }

        f.write_indent()?;
        let text = self.token().unwrap().text().to_owned();
        let text = match f.quote_style() {
            QuoteStyle::Double | QuoteStyle::Preserve => text,
            QuoteStyle::Single => {
                // TODO: Only supports simple conditions, so it needs to be changed to behavior closer to black
                if text.contains("\\") || text.contains("'") {
                    text
                } else {
                    format!("'{}'", &text[1..text.len() - 1])
                }
            }
        };
        write!(f, "{text}")?;

        if let Some(comment) = self.tailing_comment() {
            comment.fmt(f)?;
        }

        Ok(())
    }
}

impl Format for ast::LiteralString {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        for comment in self.leading_comments() {
            comment.fmt(f)?;
        }

        f.write_indent()?;
        let text = self.token().unwrap().text().to_owned();
        let text = match f.quote_style() {
            QuoteStyle::Single | QuoteStyle::Preserve => text,
            QuoteStyle::Double => {
                // TODO: Only supports simple conditions, so it needs to be changed to behavior closer to black
                if text.contains("\\") || text.contains("\"") {
                    text
                } else {
                    format!("\"{}\"", &text[1..text.len() - 1])
                }
            }
        };
        write!(f, "{text}")?;

        if let Some(comment) = self.tailing_comment() {
            comment.fmt(f)?;
        }

        Ok(())
    }
}
impl LiteralNode for ast::MultiLineBasicString {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

impl LiteralNode for ast::MultiLineLiteralString {
    fn token(&self) -> Option<syntax::SyntaxToken> {
        self.token()
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_format, FormatDefinitions};
    use config::{QuoteStyle, TomlVersion};

    test_format! {
        #[test]
        fn basic_string_value1(r#"key = "value""#) -> Ok(source);
    }

    test_format! {
        #[test]
        fn basic_string_value2(r#"key    = "value""#) -> Ok(r#"key = "value""#);
    }

    test_format! {
        #[test]
        fn basic_string_value_quote_style_single1(
            r#"key = "value""#,
            TomlVersion::default(),
            FormatDefinitions {
                quote_style: Some(QuoteStyle::Single),
                ..Default::default()
            }
        ) -> Ok(r#"key = 'value'"#);
    }

    test_format! {
        #[test]
        fn basic_string_value_quote_style_single2(
            r#"key = "'value'""#,
            TomlVersion::default(),
            FormatDefinitions {
                quote_style: Some(QuoteStyle::Single),
                ..Default::default()
            }
        ) -> Ok(source);
    }
}
