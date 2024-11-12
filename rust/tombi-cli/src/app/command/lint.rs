/// Lint TOML files.
#[derive(clap::Args, Debug)]
pub struct Args {
    /// Paths or glob patterns to TOML documents.
    ///
    /// If the only argument is "-", the standard input will be used.
    files: Vec<String>,
}

#[tracing::instrument(level = "debug")]
pub fn run(args: Args) -> Result<(), crate::Error> {
    Ok(())
}
