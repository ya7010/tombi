use lsp_types::{request::Formatting, DocumentFormattingParams, Position, Range, TextEdit};

use crate::server::state::{ServerState, State};

pub fn handle_formatting(
    state: State<ServerState>,
    _params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>, anyhow::Error> {
    Ok(Some(vec![TextEdit {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 0,
            },
        },
        new_text: "!!!!!!!!!!!!!!!!!!!".to_string(),
    }]))
}
