#[derive(clap::Subcommand)]
pub enum CodeGenCommand {
    Grammer(super::codegen_grammer::Args),
}
