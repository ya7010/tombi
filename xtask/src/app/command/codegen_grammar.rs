use crate::{
    codegen::{
        ast_node::generate_ast_node, ast_token::generate_ast_token,
        syntax_kind::generate_syntax_kind,
    },
    utils::project_root,
};
use anyhow::Context;
use ungrammar::Grammar;

/// Codegen Grammar.
#[derive(clap::Args)]
pub struct Args {}

pub fn run(_args: Args) -> Result<(), anyhow::Error> {
    let grammar = std::fs::read_to_string(project_root().join("xtask/toml.ungram"))
        .unwrap()
        .parse::<Grammar>()
        .unwrap();

    write_file(
        &generate_syntax_kind(&grammar)
            .context(format!("Failed to generate syntax kind from grammar."))?,
        &project_root().join("crates/syntax/src/generated/syntax_kind.rs"),
    );

    write_file(
        &generate_ast_node(&grammar)
            .context(format!("Failed to generate ast node from grammar."))?,
        &project_root().join("crates/ast/src/generated/ast_node.rs"),
    );

    write_file(
        &generate_ast_token(&grammar)
            .context(format!("Failed to generate ast node from grammar."))?,
        &project_root().join("crates/ast/src/generated/ast_token.rs"),
    );

    Ok(())
}

#[cfg(not(test))]
fn write_file(content: &str, target: &std::path::Path) {
    if !target.exists() {
        std::fs::create_dir_all(target.parent().unwrap())
            .unwrap_or_else(|err| panic!("Failed to create directory: {}", err));
    }
    std::fs::write(target, content)
        .unwrap_or_else(|err| panic!("Failed to write file {}: {}", target.display(), err));
}

#[cfg(test)]
fn write_file(_content: &str, _target: &std::path::Path) {}
