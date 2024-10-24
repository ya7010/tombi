mod handler;
mod router;
mod state;

use handler::Handler;
use lsp_server::Message;
use lsp_types::request::DocumentSymbolRequest;
use lsp_types::request::Formatting;
use lsp_types::request::Initialize;
use lsp_types::request::Request;
use lsp_types::request::Shutdown;
use lsp_types::OneOf;
use lsp_types::ServerCapabilities;

use crate::version::version;

pub fn run() -> Result<(), anyhow::Error> {
    tracing::info!("Tombi LSP Server Version \"{}\" will start.", version());

    let (connection, io_threads) = lsp_server::Connection::stdio();
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    })
    .unwrap();
    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };
    dbg!(&initialization_params);
    main_loop(connection);

    io_threads.join()?;
    tracing::info!("server did shut down");

    Ok(())
}

fn main_loop(connection: lsp_server::Connection) {
    let receiver = connection.receiver;
    let sender = connection.sender;
    for msg in receiver {
        match msg {
            Message::Request(request) => {
                tracing::info!("request: {:?}", request);
                match request.method.as_str() {
                    Initialize::METHOD => sender.send(Initialize::handle_with(request)),
                    Shutdown::METHOD => {
                        sender
                            .send(Shutdown::handle_with(request))
                            .map_err(|e| {
                                tracing::error!("Failed to send shutdown response: {:?}", e)
                            })
                            .ok();
                        break;
                    }
                    Formatting::METHOD => sender.send(Formatting::handle_with(request)),
                    DocumentSymbolRequest::METHOD => {
                        sender.send(DocumentSymbolRequest::handle_with(request))
                    }
                    _ => {
                        tracing::info!("No handler for request: {:?}", &request);
                        Ok(())
                    }
                }
                .map_err(|e| tracing::error!("Failed to send shutdown response: {:?}", e))
                .ok();
            }
            Message::Notification(notification) => {
                tracing::info!("notification: {:?}", notification);
                // handle_notification(&mut state, notification);
            }
            Message::Response(response) => {
                tracing::info!("response: {:?}", response);
                // state.handle_response(response);
            }
        }
    }
}
