use lsp_types::{request::Formatting, DocumentFormattingParams, Position, Range, TextEdit};

use super::Handler;

pub fn handle_formatting(
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

impl Handler for Formatting {
    type Request = Self;

    fn handle(params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>, anyhow::Error> {
        handle_formatting(params)
    }
}
