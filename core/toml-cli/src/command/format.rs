/// Format TOML files.
#[derive(clap::Args)]
pub struct Args {
    /// Set the line-length
    #[arg(long, default_value = "100")]
    line_length: u8,
}

pub fn run(_args: Args) -> Result<(), crate::Error> {
    Ok(())
}
