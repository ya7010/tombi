use crate::position::Position;

impl From<tower_lsp::lsp_types::Position> for Position {
    fn from(position: tower_lsp::lsp_types::Position) -> Self {
        Self::new(position.line, position.character)
    }
}
