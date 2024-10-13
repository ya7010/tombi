mod container;
mod error;
mod parse;
mod validation;

pub use error::Error;
use parse::Parse;
use rowan::cursor::SyntaxNode;
pub use syntax::SyntaxKind;

pub fn parse(_source: &str) -> Parse<SyntaxNode> {
    use syntax::SyntaxKind::*;
    // let mut lexer = syntax::SyntaxKind::lexer(source);
    let mut builder = rowan::GreenNodeBuilder::default();
    let errors = vec![];
    builder.start_node(ROOT.into());

    builder.finish_node();

    Parse::new(builder.finish(), errors)
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
