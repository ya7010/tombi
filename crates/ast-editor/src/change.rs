#[derive(Debug)]
pub enum Change {
    ReplaceRange {
        old: std::ops::RangeInclusive<syntax::SyntaxElement>,
        new: Vec<syntax::SyntaxElement>,
    },
}
