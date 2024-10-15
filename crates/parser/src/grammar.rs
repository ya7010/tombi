mod key_value;
mod root;
mod table;

use syntax::SyntaxKind;

use crate::output::Output;

pub fn parse(input: &crate::Input) -> Output {
    let _p = tracing::info_span!("grammar::parse").entered();
    let mut p = crate::parser::Parser::new(input);

    let m = p.start();
    root::parse(&mut p);
    m.complete(&mut p, SyntaxKind::ROOT);
    let events = p.finish();

    crate::event::process(events)
}
