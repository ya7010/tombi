pub mod format;
pub mod lint;

#[derive(clap::Subcommand)]
pub enum TomlCommand {
    #[command(alias = "fmt")]
    Format(format::Args),

    #[command(alias = "check")]
    Lint(lint::Args),
}
