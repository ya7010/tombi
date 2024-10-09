/// Format TOML files.
#[derive(clap::Args)]
pub struct Args {
    /// Set the line-length
    #[arg(long)]
    line_length: Option<u8>,
}

pub fn run(_args: Args) -> Result<(), crate::Error> {
    Ok(())
}
