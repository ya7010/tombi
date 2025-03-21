#[derive(Debug)]
pub enum Change {
    AppendTop {
        new: Vec<syntax::SyntaxElement>,
    },
    Append {
        base: syntax::SyntaxElement,
        new: Vec<syntax::SyntaxElement>,
    },
    Remove {
        target: syntax::SyntaxElement,
    },
    ReplaceRange {
        old: std::ops::RangeInclusive<syntax::SyntaxElement>,
        new: Vec<syntax::SyntaxElement>,
    },
}
