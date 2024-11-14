use syntax::SyntaxKind;

pub struct Lexed<'a> {
    pub text: &'a str,
    pub kinds: Vec<SyntaxKind>,
    pub start_offsets: Vec<u32>,
    pub errors: Vec<crate::Error>,
}

impl<'a> Lexed<'a> {
    pub fn new(text: &'a str) -> Self {
        let kinds = Vec::new();
        let start_offsets = Vec::new();
        let errors = Vec::new();

        Lexed {
            text,
            kinds,
            start_offsets,
            errors,
        }
    }
}
