mod error;
mod generated;

pub use error::{Error, SyntaxError};
pub use generated::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TomlLanguage {}

impl rowan::Language for TomlLanguage {
    type Kind = crate::SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= crate::SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, crate::SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

/// en: SyntaxNode is also known as `RedNode`.
pub type SyntaxNode = rowan::SyntaxNode<crate::TomlLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<crate::TomlLanguage>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<TomlLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<TomlLanguage>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<TomlLanguage>;

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn empty() {
        let mut lex = SyntaxKind::lexer("");

        assert_eq!(lex.next(), None);
    }

    #[test]
    fn bare_key() {
        let mut lex = SyntaxKind::lexer("test");

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
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
        println!("{:?}", lex);

        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BRACKET_END)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::NEWLINE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::NEWLINE)));
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
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::DOUBLE_BRACKET_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::DOUBLE_BRACKET_END)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::NEWLINE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::NEWLINE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::NEWLINE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::DOUBLE_BRACKET_START)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::DOUBLE_BRACKET_END)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::NEWLINE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::NEWLINE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BARE_KEY)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::EQUAL)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::WHITESPACE)));
        assert_eq!(lex.next(), Some(Ok(SyntaxKind::BASIC_STRING)));
        assert_eq!(lex.next(), None);
    }
}
