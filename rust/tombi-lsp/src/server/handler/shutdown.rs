use lsp_types::request::Shutdown;

use super::Handler;

pub fn handle_shutdown(_params: ()) -> Result<(), anyhow::Error> {
    let _p = tracing::debug_span!("handle_shutdown").entered();
    tracing::info!("Server shutting down");

    Ok(())
}

impl Handler for Shutdown {
    type Request = Self;

    fn handle(params: ()) -> Result<(), anyhow::Error> {
        handle_shutdown(params)
    }
}
