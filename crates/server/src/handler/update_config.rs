use tower_lsp::lsp_types::{
    notification::ShowMessage, MessageType, ShowMessageParams, TextDocumentIdentifier,
};

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
            match config::Config::try_from(config_url) {
                Ok(config) => {
                    let mut cfg = backend.config.write().await;
                    *cfg = config;

                    tracing::debug!("config updated");

                    return Ok(true);
                }
                Err(err) => {
                    backend
                        .client
                        .send_notification::<ShowMessage>(ShowMessageParams {
                            typ: MessageType::ERROR,
                            message: err.to_string(),
                        })
                        .await;

                    return Ok(false);
                }
            }
        }
    }

    Ok(false)
}
