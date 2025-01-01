use tower_lsp::lsp_types::TextDocumentIdentifier;

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_update_config(
    backend: &Backend,
    TextDocumentIdentifier {
        uri: config_url, ..
    }: TextDocumentIdentifier,
) -> Result<bool, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_update_config");
    let Some(workspace_folders) = backend.client.workspace_folders().await? else {
        return Ok(false);
    };

    for workspace_folder in workspace_folders {
        let Ok(workspace_config_url) = workspace_folder.uri.join("tombi.toml") else {
            continue;
        };
        if config_url == workspace_config_url {
            return Ok(true);
        }
    }

    Ok(false)
}
