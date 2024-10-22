/// Lint TOML files.
#[derive(clap::Args, Debug)]
pub struct Args {
    /// Paths or glob patterns to TOML documents.
    ///
    /// If the only argument is "-", the standard input will be used.
    files: Vec<String>,
}

pub fn run(args: Args) -> Result<(), crate::Error> {
    tracing::debug_span!("run").in_scope(|| {
        tracing::debug!("{args:?}");
        Ok(())
    })
}
