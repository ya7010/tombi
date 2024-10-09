pub mod format;
pub mod lint;

#[derive(clap::Subcommand)]
pub enum SubCommand {
    Format(format::Args),
    Lint(lint::Args),
}
