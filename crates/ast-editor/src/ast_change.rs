#[allow(dead_code)]

pub enum AstChange {
    ReplaceMany {
        elements: Vec<syntax::SyntaxElement>,
        new_elements: Vec<syntax::SyntaxElement>,
    },
}
