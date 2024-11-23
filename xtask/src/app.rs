use crate::command;
use clap::Parser;

#[derive(clap::Parser)]
#[command(name = "toml", version)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: command::XTaskCommand,
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

pub fn run(args: impl Into<Args>) -> Result<(), anyhow::Error> {
    let args = args.into();
    match args.subcommand {
        command::XTaskCommand::Codegen(subcommand) => match subcommand {
            command::CodeGenCommand::Grammar(args) => command::codegen_grammar::run(args)?,
        },
    }
    Ok(())
}
