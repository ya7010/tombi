#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionHint {
    InTableHeader,
    DotTrigger { range: text::Range },
    EqualTrigger { range: text::Range },
    SpaceTrigger { range: text::Range },
}
