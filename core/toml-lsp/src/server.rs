use lsp_server::Message;
use lsp_types::request::GotoDefinition;

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
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("toml-toolkit"),
            version: Some(crate::version().to_string()),
        }),
        capabilities: server_capabilities,
    };

    let initialize_result = serde_json::to_value(initialize_result).unwrap();

    if let Err(e) = connection.initialize_finish(initialize_id, initialize_result) {
        if e.channel_is_disconnected() {
            io_threads.join()?;
        }
        return Err(e.into());
    }

    // If the io_threads have an error, there's usually an error on the main
    // loop too because the channels are closed. Ensure we report both errors.
    match (main_loop(connection), io_threads.join()) {
        (Err(loop_e), Err(join_e)) => anyhow::bail!("{loop_e}\n{join_e}"),
        (Ok(_), Err(join_e)) => anyhow::bail!("{join_e}"),
        (Err(loop_e), Ok(_)) => anyhow::bail!("{loop_e}"),
        (Ok(_), Ok(_)) => {}
    }

    tracing::info!("server did shut down");

    Ok(())
}

fn main_loop(connection: lsp_server::Connection) -> Result<(), anyhow::Error> {
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                tracing::debug!("request: {:?}", req);
                if connection.handle_shutdown(&req)? {
                    break;
                }
                match cast::<lsp_types::request::Formatting>(req) {
                    Ok((id, params)) => {
                        tracing::debug!("got gotoDefinition request #{id}: {params:?}");
                    }
                    Err(err @ lsp_server::ExtractError::JsonError { .. }) => {
                        tracing::debug!("ExtractError::JsonError: {:?}", err);
                        break;
                    }
                    Err(lsp_server::ExtractError::MethodMismatch(req)) => {
                        tracing::debug!("ExtractError::MethodMismatch: {:?}", req)
                    }
                };
            }
            Message::Notification(notification) => {
                tracing::debug!("notification: {:?}", notification);
                // handle_notification(&mut state, notification);
            }
            Message::Response(response) => {
                tracing::debug!("response: {:?}", response);
                // state.handle_response(response);
            }
        }
    }

    Ok(())
}

fn cast<R>(
    req: lsp_server::Request,
) -> Result<(lsp_server::RequestId, R::Params), lsp_server::ExtractError<lsp_server::Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
