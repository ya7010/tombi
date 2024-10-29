impl From<tower_lsp::lsp_types::Position> for Position {
    fn from(position: tower_lsp::lsp_types::Position) -> Self {
        Self {
            line: position.line,
            column: position.character,
        }
    }
}
