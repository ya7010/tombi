pub mod format;
pub mod lint;

#[derive(clap::Subcommand)]
pub enum TomlCommand {
    Format(format::Args),
    Lint(lint::Args),
}
