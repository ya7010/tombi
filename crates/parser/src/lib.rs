mod container;
mod error;
mod parse;
mod validation;

pub use error::Error;
use logos::Logos;
use parse::Parse;
use rowan::cursor::SyntaxNode;
pub use syntax::SyntaxKind;

pub fn parse(source: &str) -> Parse<SyntaxNode> {
    use syntax::SyntaxKind::*;
    let mut lexer = syntax::SyntaxKind::lexer(source);
    let mut builder = rowan::GreenNodeBuilder::default();
    let mut errors = vec![];
    builder.start_node(ROOT.into());
    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => match token {
                ROOT => {
                    unreachable!("unexpected root token");
                }
                COMMENT => {
                    // TODO: need allowed_comment_chars
                    builder.token(token.into(), lexer.slice());
                }
                NEWLINE => {
                    builder.token(token.into(), lexer.slice());
                }
                BARE_KEY => {
                    let value = lexer.slice();
                    builder.token(token.into(), value);
                }
                EQUAL => {
                    builder.token(token.into(), lexer.slice());
                }
                BASIC_STRING
                | MULTI_LINE_BASIC_STRING
                | LITERAL_STRING
                | MULTI_LINE_LITERAL_STRING
                | INTEGER_DEC
                | INTEGER_HEX
                | INTEGER_OCT
                | INTEGER_BIN
                | FLOAT
                | BOOLEAN
                | OFFSET_DATE_TIME
                | LOCAL_DATE_TIME
                | LOCAL_DATE
                | LOCAL_TIME => {
                    let value = lexer.slice();
                    builder.token(token.into(), value);
                }
                _ => continue,
            },
            Err(error) => {
                let span = lexer.span();
                errors.push(crate::Error::InvalidToken { error, span });
            }
        }
    }

    builder.finish_node();

    Parse::new(builder.finish(), vec![])
}

/// Matches a `SyntaxNode` against an `ast` type.
///
/// # Example:
///
/// ```ignore
/// match_ast! {
///     match node {
///         ast::CallExpr(it) => { ... },
///         ast::MethodCallExpr(it) => { ... },
///         ast::MacroCall(it) => { ... },
///         _ => None,
///     }
/// }
/// ```
#[macro_export]
macro_rules! match_ast {
    (match $node:ident { $($tt:tt)* }) => { $crate::match_ast!(match ($node) { $($tt)* }) };

    (match ($node:expr) {
        $( $( $path:ident )::+ ($it:pat) => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = $($path::)+cast($node.clone()) { $res } else )*
        { $catch_all }
    }};
}
