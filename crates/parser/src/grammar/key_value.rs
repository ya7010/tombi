use super::{leading_comments, tailing_comment, Grammer};
use crate::parser::Parser;
use syntax::{SyntaxKind::*, T};

impl Grammer for ast::KeyValue {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        ast::Keys::parse(p);

        if !p.eat(T![=]) {
            p.bump_any();
            p.error(crate::Error::ExpectedEqual);
        }

        ast::Value::parse(p);

        tailing_comment(p);

        m.complete(p, KEY_VALUE);
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use syntax::SyntaxError;
    use text::{Column, Line};

    #[rstest]
    #[case("key1 = # INVALID", &[(crate::Error::ExpectedValue, ((0, 6), (0, 7)))])]
    #[case("key1 = 2024-01-00T", &[(crate::Error::ExpectedValue, ((0, 7), (0, 18)))])]
    #[case(r#"
key1 = 1
key2 = # INVALID
key3 = 3
"#.trim_start(), &[(crate::Error::ExpectedValue, ((1, 6), (1, 7)))])]
    #[case(r#"
key1 = "str"
key2 = "invalid
key3 = 1
"#.trim_start(), &[(crate::Error::ExpectedValue, ((1, 8), (1, 15)))]
    )]
    #[case(r#"
key1 = "str"
key2 = invalid"
key3 = 1
"#.trim_start(), &[(crate::Error::ExpectedValue, ((1, 7), (1, 15)))]
    )]
    #[case(r#"
key1 = 'str'
key2 = 'invalid
key3 = 1
"#.trim_start(), &[(crate::Error::ExpectedValue, ((1, 8), (1, 15)))]
    )]
    #[case(r#"
key1 = 'str'
key2 = invalid'
key3 = 1
"#.trim_start(), &[(crate::Error::ExpectedValue, ((1, 7), (1, 15)))]
    )]
    #[case(r#"
key1 "value"
key2 = 1
"#.trim_start(), &[
    (crate::Error::ExpectedEqual, ((0, 5), (0, 12))),
    (crate::Error::ExpectedValue, ((0, 5), (0, 12)))
]
    )]
    fn invalid_key_value(
        #[case] source: &str,
        #[case] errors: &[(crate::Error, ((Line, Column), (Line, Column)))],
    ) {
        let p = crate::parse(source);

        assert_eq!(
            p.errors(),
            errors
                .into_iter()
                .map(|(error, range)| SyntaxError::new(*error, (*range).into()))
                .collect::<Vec<_>>()
        );
    }
}
