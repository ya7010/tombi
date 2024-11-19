mod builder;
mod error;
mod event;
mod grammar;
mod input;
mod lexed;
mod marker;
mod output;
mod parsed;
mod parser;
mod token_set;
mod validation;

pub use error::Error;
pub use event::Event;
use grammar::Parse;
use input::Input;
use lexed::lex;
pub use lexed::LexedStr;
use output::Output;
use parsed::Parsed;
pub use syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

pub fn parse(source: &str) -> Parsed<SyntaxNode> {
    parse_as::<ast::Root>(source)
}

#[allow(private_bounds)]
pub fn parse_as<G: Parse>(source: &str) -> Parsed<SyntaxNode> {
    let lexed = lex(source);
    let input = lexed.to_input();
    let output = grammar::parse::<G>(&input);
    let (green_tree, errors) = build_green_tree(&lexed, output);

    Parsed::new(green_tree, errors)
}

pub fn build_green_tree(
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

#[cfg(test)]
#[macro_export]
macro_rules! test_parser {
    {#[test] fn $name:ident($source:expr) -> Err([$(SyntaxError($error:ident, $line1:literal:$column1:literal..$line2:literal:$column2:literal)),*$(,)*])} => {
        #[test]
        fn $name() {
            let p = crate::parse(textwrap::dedent($source).trim());

            assert_eq!(p.errors(), vec![$(syntax::SyntaxError::new($error, (($line1, $column1), ($line2, $column2)).into())),*])
        }
    };
}
