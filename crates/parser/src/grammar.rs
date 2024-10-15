mod key_value;
mod root;
mod table;

use crate::output::Output;

pub fn parse(input: &crate::Input) -> Output {
    let _p = tracing::info_span!("grammar::parse").entered();
    let mut p = crate::parser::Parser::new(input);

    root::parse(&mut p);

    let events = p.finish();

    crate::event::process(events)
}
