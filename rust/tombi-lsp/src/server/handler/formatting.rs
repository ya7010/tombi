use tower_lsp::lsp_types::{DocumentFormattingParams, TextEdit};

use crate::server::backend::Backend;

pub async fn handle_formatting(
    backend: &Backend,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>, tower_lsp::jsonrpc::Error> {
    // Ok(Some(vec![TextEdit {
    //     range: Range {
    //         start: Position {
    //             line: 0,
    //             character: 0,
    //         },
    //         end: Position {
    //             line: 0,
    //             character: 0,
    //         },
    //     },
    //     new_text: "!!!!!!!!!!!!!!!!!!!".to_string(),
    // }]))

    Ok(None)
}
