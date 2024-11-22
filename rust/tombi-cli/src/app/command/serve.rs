pub use tombi_lsp::server::Args;

pub fn run(args: impl Into<Args>) -> Result<(), crate::Error> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(tombi_lsp::server::run(args))?;

    Ok(())
}
