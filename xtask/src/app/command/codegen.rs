#[derive(clap::Subcommand)]
pub enum CodeGenCommand {
    Grammar(super::codegen_grammar::Args),
}
