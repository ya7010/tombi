use crate::utils::project_root;
use ungrammar::Grammar;

/// Codegen Grammer.
#[derive(clap::Args)]
pub struct Args {}

pub fn run(_args: Args) -> Result<(), crate::Error> {
    let _grammar = std::fs::read_to_string(project_root().join("xtask/toml.ungram"))
        .unwrap()
        .parse::<Grammar>()
        .unwrap();
    Ok(())
}
