use tower_lsp::lsp_types::{DocumentFormattingParams, Position, Range, TextEdit};

pub async fn handle_formatting(
    _params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>, tower_lsp::jsonrpc::Error> {
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
