use tower_lsp::lsp_types::{DocumentFormattingParams, TextEdit};

use crate::server::backend::Backend;

pub async fn handle_formatting(
    backend: &Backend,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>, tower_lsp::jsonrpc::Error> {
    Ok(None)
}
