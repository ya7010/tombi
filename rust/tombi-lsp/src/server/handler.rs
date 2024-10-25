mod document_symbol;
mod formatting;
mod initialize;
mod shutdown;

pub use document_symbol::handle_document_symbol;
pub use formatting::handle_formatting;
pub use initialize::handle_initialize;
pub use shutdown::handle_shutdown;

use super::state::State;

pub fn handle<H, S, R>(handler: H, state: S, request: lsp_server::Request) -> lsp_server::Message
where
    R: lsp_types::request::Request,
    H: FnOnce(State<S>, R::Params) -> Result<R::Result, anyhow::Error>,
{
    let _p = tracing::debug_span!("handle").entered();
    tracing::debug!("Handling request: {:#?}", request);

    let request_id = request.id.clone();
    let request_params = match serde_json::from_value::<R::Params>(request.params) {
        Ok(params) => params,
        Err(err) => {
            let response = lsp_server::Response::new_err(request_id, 1, err.to_string());
            let message = lsp_server::Message::Response(response);
            return message;
        }
    };
    match handler(State(state), request_params) {
        Ok(result) => {
            let response = lsp_server::Response::new_ok(request_id, result);
            let message = lsp_server::Message::Response(response);
            message
        }
        Err(err) => {
            let response = lsp_server::Response::new_err(request_id, 1, err.to_string());
            let message = lsp_server::Message::Response(response);
            message
        }
    }
}
