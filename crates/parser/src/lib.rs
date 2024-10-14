mod container;
mod error;
mod event;
mod input;
mod marker;
mod output;
mod parse;
mod parser;
mod validation;

pub use error::Error;
pub use event::Event;
use logos::{Lexer, Logos};
use parse::Parse;
use rowan::cursor::SyntaxNode;
pub use syntax::SyntaxKind;

pub fn parse(source: &str) -> Parse<SyntaxNode> {
    let lex = syntax::SyntaxKind::lexer(source);
    let (tree, errors) = build_tree(lex);

    Parse::new(tree, errors)
}

pub fn build_tree(_lex: Lexer<'_, SyntaxKind>) -> (rowan::GreenNode, Vec<syntax::SyntaxError>) {
    let mut builder = syntax::SyntaxTreeBuilder::default();

    builder.start_node(SyntaxKind::ROOT.into());

    builder.finish_node();

    builder.finish()
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
