mod arg;
mod command;

use clap::Parser;
use clap_verbosity_flag::Verbosity;

/// TOML: TOML linter and code formatter.
#[derive(clap::Parser)]
#[command(name = "toml", version)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: command::TomlCommand,
    #[command(flatten)]
    verbose: Verbosity,
}

impl<I, T> From<I> for Args
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    #[inline]
    fn from(value: I) -> Self {
        Self::parse_from(value)
    }
}

pub fn run(args: impl Into<Args>) -> Result<(), crate::Error> {
    let args = args.into();
    match args.subcommand {
        command::TomlCommand::Format(args) => command::format::run(args),
        command::TomlCommand::Lint(args) => command::lint::run(args),
    }
}
