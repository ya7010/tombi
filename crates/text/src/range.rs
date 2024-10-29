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

impl Range {
    #[inline]
    pub const fn new(start: Position, end: Position) -> Range {
        Range { start, end }
    }

    #[inline]
    pub const fn empty(pos: Position) -> Range {
        Range {
            start: pos,
            end: pos,
        }
    }
}
