#[derive(Debug, clap::Subcommand, Default)]
pub enum CodeGenCommand {
    #[default]
    All,
    Grammar,
    Jsonschema,
}
