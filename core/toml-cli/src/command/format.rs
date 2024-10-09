/// Format TOML files.
#[derive(clap::Args)]
pub struct Args {
    /// Set the line-length
    #[arg(long, default_value = formatter::DEFAULT_LINE_LENGTH_STR)]
    line_length: u8,
}

pub fn run(_args: Args) -> Result<(), crate::Error> {
    formatter::format("", &formatter::Options::default());
    Ok(())
}
