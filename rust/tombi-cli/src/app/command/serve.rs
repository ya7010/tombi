pub use server::Args;

pub fn run(args: impl Into<Args>, offline: bool) -> Result<(), crate::Error> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(server::serve(args, offline));

    Ok(())
}
