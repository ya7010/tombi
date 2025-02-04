#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionHint {
    InTableHeader,
    InArray,
    DotTrigger { range: text::Range },
    EqualTrigger { range: text::Range },
}
