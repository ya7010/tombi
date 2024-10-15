use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct Context<'a> {
    pub options: Cow<'a, crate::Options>,
}
