mod backend;
mod document;
mod document_symbol;
mod handler;
mod toml;

/// Run TOML Language Server
#[derive(clap::Args, Debug)]
pub struct Args {}

pub async fn serve(_args: impl Into<Args>) -> Result<(), anyhow::Error> {
    tracing::info!(
        "Tombi LSP Server Version \"{}\" will start.",
        env!("CARGO_PKG_VERSION")
    );

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = tower_lsp::LspService::build(crate::backend::Backend::new).finish();

    tower_lsp::Server::new(stdin, stdout, socket)
        .serve(service)
        .await;

    tracing::info!("Tombi LSP Server did shut down.");

    Ok(())
}
