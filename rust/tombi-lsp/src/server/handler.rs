mod document_symbol;
mod formatting;
mod initialize;
mod shutdown;

pub use serde::de::DeserializeOwned;

pub trait Handler {
    type Request: lsp_types::request::Request;

    fn handle(
        request: <Self::Request as lsp_types::request::Request>::Params,
    ) -> Result<<Self::Request as lsp_types::request::Request>::Result, anyhow::Error>;

    fn handle_with(request: lsp_server::Request) -> lsp_server::Message
    where
        <Self::Request as lsp_types::request::Request>::Params: DeserializeOwned,
    {
        let _p = tracing::debug_span!("handle_with").entered();
        tracing::debug!("Handling request: {:#?}", request);

        let request_id = request.id.clone();
        let request_params = match serde_json::from_value::<
            <Self::Request as lsp_types::request::Request>::Params,
        >(request.params)
        {
            Ok(params) => params,
            Err(err) => {
                let response = lsp_server::Response::new_err(request_id, 1, err.to_string());
                let message = lsp_server::Message::Response(response);
                return message;
            }
        };

        match Self::handle(request_params) {
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
}
