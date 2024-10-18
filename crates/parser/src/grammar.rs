mod array;
mod array_of_table;
mod inline_table;
mod key_value;
mod root;
mod table;

use root::parse_root;

use crate::output::Output;

pub fn parse(input: &crate::Input) -> Output {
    let _p = tracing::info_span!("grammar::parse").entered();
    let mut p = crate::parser::Parser::new(input);

    parse_root(&mut p);

    let events = p.finish();

    crate::event::process(events)
}

fn line_end(p: &mut crate::parser::Parser<'_>) {
    while p.eat(crate::SyntaxKind::NEWLINE) || p.eat(crate::SyntaxKind::COMMENT) {}
}
