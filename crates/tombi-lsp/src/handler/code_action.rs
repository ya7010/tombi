use tower_lsp::lsp_types::{CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams};

use crate::Backend;

pub async fn handle_code_action(
    backend: &Backend,
    params: CodeActionParams,
) -> Result<Option<Vec<CodeActionOrCommand>>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_code_action");
    tracing::trace!(?params);

    Ok(Some(vec![CodeAction {
        title: "Example Code Action".to_string(),
        kind: Some(CodeActionKind::REFACTOR_REWRITE),
        diagnostics: None,
        edit: None,
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    }
    .into()]))
}
