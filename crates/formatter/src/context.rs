use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct Context<'a> {
    #[allow(dead_code)]
    pub options: Cow<'a, crate::Options>,
}
