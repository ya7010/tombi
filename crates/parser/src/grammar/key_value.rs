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
    use syntax::SyntaxError;

    #[test]
    fn key_value_value_not_found() {
        let p = crate::parse("key1 = # INVALID");

        assert_eq!(
            p.errors(),
            vec![SyntaxError::new(
                crate::Error::ExpectedValue,
                ((0, 6), (0, 7)).into()
            )]
        );
    }

    #[test]
    fn multiple_key_value_value_not_found() {
        let p = crate::parse(
            r#"
key1 = 1
key2 = # INVALID
key3 = 3
"#
            .trim_start(),
        );

        dbg!(p.syntax_node());

        assert_eq!(
            p.errors(),
            vec![SyntaxError::new(
                crate::Error::ExpectedValue,
                ((1, 6), (1, 7)).into()
            )]
        );
    }
}
