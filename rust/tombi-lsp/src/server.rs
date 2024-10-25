mod handler;
mod state;

use handler::handle;
use handler::handle_document_symbol;
use handler::handle_formatting;
use handler::handle_initialize;
use handler::handle_shutdown;
use lsp_server::Message;
use lsp_types::request::DocumentSymbolRequest;
use lsp_types::request::Formatting;
use lsp_types::request::Initialize;
use lsp_types::request::Request;
use lsp_types::request::Shutdown;

use crate::version::version;

pub fn run() -> Result<(), anyhow::Error> {
    tracing::info!("Tombi LSP Server Version \"{}\" will start.", version());

    let (connection, io_threads) = lsp_server::Connection::stdio();

    main_loop(connection);

    io_threads.join()?;
    tracing::info!("server did shut down");

    Ok(())
}

fn main_loop(connection: lsp_server::Connection) {
    let receiver = connection.receiver;
    let sender = connection.sender;
    let state = state::ServerState {
        client_capabilities: Default::default(),
    };

    for msg in receiver {
        match msg {
            Message::Request(request) => {
                tracing::info!("request: {:?}", request);
                match request.method.as_str() {
                    Initialize::METHOD => sender.send(handle::<_, _, Initialize>(
                        handle_initialize,
                        state.clone(),
                        request,
                    )),
                    Shutdown::METHOD => {
                        sender
                            .send(handle::<_, _, Shutdown>(
                                handle_shutdown,
                                state.clone(),
                                request,
                            ))
                            .map_err(|e| {
                                tracing::error!("Failed to send shutdown response: {:?}", e)
                            })
                            .ok();
                        break;
                    }
                    Formatting::METHOD => sender.send(handle::<_, _, Formatting>(
                        handle_formatting,
                        state.clone(),
                        request,
                    )),
                    DocumentSymbolRequest::METHOD => {
                        sender.send(handle::<_, _, DocumentSymbolRequest>(
                            handle_document_symbol,
                            state.clone(),
                            request,
                        ))
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
