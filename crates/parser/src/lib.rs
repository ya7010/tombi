mod builder;
mod error;
mod event;
mod grammar;
mod input;
mod lexed;
mod marker;
mod output;
mod parse;
mod parser;
mod step;
mod token_set;
mod validation;

pub use error::Error;
pub use event::Event;
use input::Input;
use lexed::lex;
pub use lexed::LexedStr;
use output::Output;
use parse::Parse;
use rowan::cursor::SyntaxNode;
pub use syntax::SyntaxKind;

pub fn parse(source: &str) -> Parse<SyntaxNode> {
    let lexed = lex(source);
    dbg!(&lexed);
    let input = lexed.to_input();
    dbg!(&input);
    let output: Output = grammar::parse(&input);
    let (tree, errors) = build_tree(&lexed, output);

    Parse::new(tree, errors)
}

pub fn build_tree(
    lexed: &LexedStr<'_>,
    parser_output: crate::Output,
) -> (rowan::GreenNode, Vec<syntax::SyntaxError>) {
    let _p = tracing::info_span!("build_tree").entered();
    let mut builder = syntax::SyntaxTreeBuilder::default();

    dbg!(lexed);
    dbg!(&parser_output);
    let is_eof = lexed.intersperse_trivia(&parser_output, &mut |step| match step {
        step::StrStep::Token { kind, text } => builder.token(kind, text),
        step::StrStep::Enter { kind } => builder.start_node(kind),
        step::StrStep::Exit => builder.finish_node(),
        step::StrStep::Error { msg, pos } => builder.error(msg.to_owned(), pos.try_into().unwrap()),
    });

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
