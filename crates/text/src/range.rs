use crate::position::Position;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Range {
    // Invariant: start <= end
    start: Position,
    end: Position,
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
