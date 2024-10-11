pub mod codegen;
pub mod codegen_grammer;
pub use codegen::CodeGenCommand;

#[derive(clap::Subcommand)]
pub enum XTaskCommand {
    #[clap(subcommand)]
    Codegen(CodeGenCommand),
}
