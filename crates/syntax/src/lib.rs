mod builder;
mod error;
mod generated;

pub use builder::SyntaxTreeBuilder;
pub use error::{Error, SyntaxError};
pub use generated::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TomlLanguage {}

impl tombi_rowan::Language for TomlLanguage {
    type Kind = crate::SyntaxKind;

    fn kind_from_raw(raw: tombi_rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= crate::SyntaxKind::__LAST as u16);
        unsafe { std::mem::transmute::<u16, crate::SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> tombi_rowan::SyntaxKind {
        kind.into()
    }
}

/// en: SyntaxNode is also known as `RedNode`.
pub type SyntaxNode = tombi_rowan::RedNode<crate::TomlLanguage>;
pub type SyntaxToken = tombi_rowan::RedToken<crate::TomlLanguage>;
pub type SyntaxElement = tombi_rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
pub type SyntaxNodeChildren = tombi_rowan::SyntaxNodeChildren<TomlLanguage>;
pub type SyntaxElementChildren = tombi_rowan::SyntaxElementChildren<TomlLanguage>;
pub type PreorderWithTokens = tombi_rowan::api::PreorderWithTokens<TomlLanguage>;

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;
    use rstest::rstest;

    #[test]
    fn empty() {
        let mut lex = SyntaxKind::lexer("");

        assert_eq!(lex.next(), None);
    }

    #[test]
    fn bare_key() {
        let mut lex = SyntaxKind::lexer("test");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn float_dot_key() {
        let mut lex = SyntaxKind::lexer(r#"3.14159 = "pi""#);

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::FLOAT)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), None);
    }

    #[rstest]
    #[case("odt1 = 1979-05-27T07:32:00Z")]
    #[case("odt2 = 1979-05-27T00:32:00-07:00")]
    #[case("odt3 = 1979-05-27T00:32:00.999999-07:00")]
    #[case("odt4 = 1979-05-27 07:32:00Z")]
    fn offset_date_time(#[case] source: &str) {
        let mut lex = SyntaxKind::lexer(source);

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::OFFSET_DATE_TIME)));
        assert_eq!(lex.next(), None);
    }

    #[rstest]
    #[case("ldt1 = 1979-05-27T07:32:00")]
    #[case("ldt2 = 1979-05-27T00:32:00.999999")]
    fn local_date_time(#[case] source: &str) {
        let mut lex = SyntaxKind::lexer(source);

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LOCAL_DATE_TIME)));
        assert_eq!(lex.next(), None);
    }

    #[rstest]
    #[case("ld1 = 1979-05-27")]
    fn local_date(#[case] source: &str) {
        let mut lex = SyntaxKind::lexer(source);

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LOCAL_DATE)));
        assert_eq!(lex.next(), None);
    }

    #[rstest]
    #[case("lt1 = 07:32:00")]
    #[case("lt2 = 00:32:00.999999")]
    fn local_time(#[case] source: &str) {
        let mut lex = SyntaxKind::lexer(source);

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LOCAL_TIME)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn dotted_keys() {
        let mut lex = SyntaxKind::lexer(r#"apple.type = "fruit""#);

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::DOT)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn table() {
        let source = r#"
[package]
name = "toml"
version = "0.5.8"
"#
        .trim();

        let mut lex = SyntaxKind::lexer(source);

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_END)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LINE_BREAK)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LINE_BREAK)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn inline_table() {
        let mut lex = SyntaxKind::lexer("key1 = { key2 = 'value' }");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACE_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LITERAL_STRING)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACE_END)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn invalid_source() {
        let mut lex = SyntaxKind::lexer("key1 = { key2 = 'value");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACE_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Err(crate::Error::InvalidToken)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn array_of_table() {
        let source = r#"
[[package]]
name = "toml"
version = "0.5.8"

[[package]]
name = "json"
version = "1.2.4"
"#
        .trim();
        let mut lex = SyntaxKind::lexer(source);
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_END)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_END)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LINE_BREAK)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LINE_BREAK)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LINE_BREAK)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LINE_BREAK)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_END)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_END)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LINE_BREAK)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::LINE_BREAK)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), None);
    }
}
