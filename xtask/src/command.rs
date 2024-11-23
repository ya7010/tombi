pub mod codegen;
pub mod codegen_grammar;
pub mod dist;
pub use codegen::CodeGenCommand;

#[derive(clap::Subcommand)]
pub enum XTaskCommand {
    /// Generate code.
    #[clap(subcommand)]
    Codegen(CodeGenCommand),

    /// Prepare the distribution.
    Dist(dist::Args),
}
