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
use grammar::Grammer;
use input::Input;
use lexed::lex;
pub use lexed::LexedStr;
use output::Output;
use parse::Parse;
pub use syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

pub fn parse(source: &str) -> Parse<SyntaxNode> {
    parse_as::<ast::Root>(source)
}

#[allow(private_bounds)]
pub fn parse_as<G: Grammer>(source: &str) -> Parse<SyntaxNode> {
    let lexed = lex(source);
    let input = lexed.to_input();
    let output = grammar::parse::<G>(&input);
    let (tree, errors) = build_tree(&lexed, output);

    Parse::new(tree, errors)
}

pub fn build_tree(
    lexed: &LexedStr<'_>,
    parser_output: crate::Output,
) -> (tombi_rowan::GreenNode, Vec<syntax::SyntaxError>) {
    let _p = tracing::info_span!("build_tree").entered();
    let mut builder = syntax::SyntaxTreeBuilder::default();
    let mut enter_pos = 0;

    let _ = lexed.intersperse_trivia(&parser_output, &mut |step| match step {
        step::StrStep::Token { kind, text } => builder.token(kind, text),
        step::StrStep::Enter { kind, pos } => {
            builder.start_node(kind);
            enter_pos = pos as u32;
        }
        step::StrStep::Exit => builder.finish_node(),
        step::StrStep::Error { error, pos } => {
            builder.error(error.to_string(), enter_pos..(pos as u32))
        }
    });

    builder.finish()
}
