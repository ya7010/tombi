use clap::Parser;

/// TOML: TOML linter and code formatter.
#[derive(clap::Parser)]
#[command(name = "toml", version)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: crate::command::SubCommand,
}

impl<I, T> From<I> for Args
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    fn from(value: I) -> Self {
        Self::parse_from(value)
    }
}

pub fn run(args: impl Into<Args>) -> Result<(), crate::Error> {
    let args = args.into();
    match args.subcommand {
        crate::command::SubCommand::Format(args) => crate::command::format::run(args),
        crate::command::SubCommand::Lint(args) => crate::command::lint::run(args),
    }
}
