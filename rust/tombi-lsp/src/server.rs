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
use lsp_types::DocumentChanges;
use state::ServerState;

use crate::{from_json, lsp};

pub fn run() -> Result<(), anyhow::Error> {
    tracing::info!("server version {} will start", 1);

    let (connection, io_threads) = lsp_server::Connection::stdio();

    main_loop(connection, io_threads);

    tracing::info!("server did shut down");

    Ok(())
}

fn main_loop(connection: lsp_server::Connection, io_threads: lsp_server::IoThreads) {
    let receiver = connection.receiver;
    let sender = connection.sender;
    for msg in receiver {
        match msg {
            Message::Request(request) => {
                tracing::debug!("request: {:?}", request);
                match request.method.as_str() {
                    Initialize::METHOD => Initialize::handle_with(sender.clone(), request),
                    Shutdown::METHOD => {
                        Shutdown::handle_with(sender.clone(), request);
                        break;
                    }
                    Formatting::METHOD => {
                        tracing::info!("Formatting request: {:?}", request);
                    }
                    DocumentSymbolRequest::METHOD => {
                        DocumentSymbolRequest::handle_with(sender.clone(), request)
                    }
                    _ => {
                        tracing::debug!("No handler for request: {:?}", &request);
                    }
                }
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
}
