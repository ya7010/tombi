use super::{leading_comments, tailing_comment, Grammer};
use crate::parser::Parser;
use syntax::{SyntaxKind::*, T};

impl Grammer for ast::KeyValue {
    fn parse(p: &mut Parser<'_>) {
        let m = p.start();

        leading_comments(p);

        ast::Keys::parse(p);

        if !p.eat(T![=]) {
            p.error(crate::Error::ExpectedEquals);
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
    #[case("key1 = # INVALID", crate::Error::ExpectedValue, ((0, 6), (0, 7)))]
    #[case(r#"
key1 = 1
key2 = # INVALID
key3 = 3
"#.trim_start(), crate::Error::ExpectedValue, ((1, 6), (1, 7)))]
    fn invalid_key_value(
        #[case] source: &str,
        #[case] error: crate::Error,
        #[case] range: ((Line, Column), (Line, Column)),
    ) {
        let p = crate::parse(source);

        assert_eq!(p.errors(), vec![SyntaxError::new(error, range.into())]);
    }
}
