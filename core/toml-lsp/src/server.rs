use std::path::PathBuf;

use camino::Utf8PathBuf;

use crate::{from_json, lsp};

pub fn run() -> Result<(), anyhow::Error> {
    tracing::info!("server version {} will start", 1);

    let (connection, io_threads) = lsp_server::Connection::stdio();

    let (initialize_id, initialize_params) = match connection.initialize_start() {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };

    tracing::info!("{initialize_id}");
    tracing::info!("InitializeParams: {}", initialize_params);

    let lsp_types::InitializeParams {
        capabilities,
        workspace_folders,
        initialization_options,
        client_info,
        ..
    } = from_json::<lsp_types::InitializeParams>("InitializeParams", &initialize_params)?;

    if let Some(client_info) = client_info {
        tracing::info!(
            "Client '{}' {}",
            client_info.name,
            client_info.version.as_deref().unwrap_or_default()
        );
        if let Some(vscode_version) = client_info
            .name
            .starts_with("Visual Studio Code")
            .then(|| {
                client_info
                    .version
                    .as_deref()
                    .map(semver::Version::parse)
                    .and_then(Result::ok)
            })
            .flatten()
        {
            tracing::info!("VSCode Version: {:?}", vscode_version);
        }
    }

    let workspace_uris = workspace_folders
        .map(|workspaces| workspaces.into_iter().map(|it| it.uri).collect::<Vec<_>>());

    tracing::debug!("Workspace folders: {:?}", workspace_uris);

    tracing::info!("server did shut down");
    let client_capabilities = capabilities;
    let server_capabilities = lsp::server_capabilities(&client_capabilities);

    let initialize_result = lsp_types::InitializeResult {
        capabilities: server_capabilities,
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("toml-toolkit"),
            version: Some(crate::version().to_string()),
        }),
    };

    Ok(())
}
