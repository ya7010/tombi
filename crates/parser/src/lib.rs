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
) -> (rg_tree::GreenNode, Vec<syntax::SyntaxError>) {
    let _p = tracing::info_span!("build_tree").entered();
    let mut builder = syntax::SyntaxTreeBuilder::default();
    let mut enter_position = Default::default();

    let _ = lexed.intersperse_trivia(&parser_output, &mut |step| match step {
        lexed::Step::Token {
            kind,
            text,
            position,
        } => {
            builder.token(kind, text);
            enter_position = position;
        }
        lexed::Step::Enter { kind } => {
            builder.start_node(kind);
        }
        lexed::Step::Exit => builder.finish_node(),
        lexed::Step::Error { error, position } => {
            builder.error(error.to_string(), (enter_position, position).into())
        }
    });

    builder.finish()
}
