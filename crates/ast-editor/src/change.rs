#[derive(Debug)]
pub enum Change {
    Append {
        base: syntax::SyntaxElement,
        new: syntax::SyntaxElement,
    },
    Remove {
        target: syntax::SyntaxElement,
    },
    ReplaceRange {
        old: std::ops::RangeInclusive<syntax::SyntaxElement>,
        new: Vec<syntax::SyntaxElement>,
    },
}
