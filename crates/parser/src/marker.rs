use drop_bomb::DropBomb;
use syntax::SyntaxKind;

use crate::{parser::Parser, Event};

pub(crate) struct Marker {
    event_index: u32,
    bomb: DropBomb,
}

impl Marker {
    pub fn new(event_index: u32) -> Marker {
        Marker {
            event_index,
            bomb: DropBomb::new("Marker must be either completed or abandoned"),
        }
    }

    /// Finishes the syntax tree node and assigns `kind` to it,
    /// and mark the create a `CompletedMarker` for possible future
    /// operation like `.precede()` to deal with forward_parent.
    pub(crate) fn complete(mut self, p: &mut Parser<'_>, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();
        let idx = self.event_index as usize;
        match &mut p.events[idx] {
            Event::Start { kind: slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        p.push_event(Event::Finish);
        CompletedMarker::new(self.event_index, kind)
    }

    /// Abandons the syntax tree node. All its children
    /// are attached to its parent instead.
    pub(crate) fn abandon(mut self, p: &mut Parser<'_>) {
        self.bomb.defuse();
        let idx = self.event_index as usize;
        if idx == p.events.len() - 1 {
            match p.events.pop() {
                Some(Event::Start {
                    kind: SyntaxKind::TOMBSTONE,
                    forward_parent: None,
                }) => (),
                _ => unreachable!(),
            }
        }
    }
}

pub(crate) struct CompletedMarker {
    event_index: u32,
    kind: SyntaxKind,
}

impl CompletedMarker {
    fn new(event_index: u32, kind: SyntaxKind) -> Self {
        CompletedMarker { event_index, kind }
    }

    /// This method allows to create a new node which starts
    /// *before* the current one. That is, parser could start
    /// node `A`, then complete it, and then after parsing the
    /// whole `A`, decide that it should have started some node
    /// `B` before starting `A`. `precede` allows to do exactly
    /// that. See also docs about
    /// [`Event::Start::forward_parent`](crate::event::Event::Start::forward_parent).
    ///
    /// Given completed events `[START, FINISH]` and its corresponding
    /// `CompletedMarker(pos: 0, _)`.
    /// Append a new `START` events as `[START, FINISH, NEWSTART]`,
    /// then mark `NEWSTART` as `START`'s parent with saving its relative
    /// distance to `NEWSTART` into forward_parent(=2 in this case);
    pub(crate) fn precede(self, p: &mut Parser<'_>) -> Marker {
        let new_m = p.start();
        let idx = self.event_index as usize;
        match &mut p.events[idx] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = Some(new_m.event_index - self.event_index);
            }
            _ => unreachable!(),
        }
        new_m
    }

    /// Extends this completed marker *to the left* up to `m`.
    pub(crate) fn extend_to(self, p: &mut Parser<'_>, mut m: Marker) -> CompletedMarker {
        m.bomb.defuse();
        let idx = m.event_index as usize;
        match &mut p.events[idx] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = Some(self.event_index - m.event_index);
            }
            _ => unreachable!(),
        }
        self
    }

    pub(crate) fn kind(&self) -> SyntaxKind {
        self.kind
    }
}
