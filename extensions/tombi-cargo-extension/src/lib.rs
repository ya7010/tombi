mod goto_declaration;
mod goto_definition;

pub use goto_declaration::goto_declaration;
pub use goto_definition::goto_definition;
use tombi_ast::AstNode;
use tombi_config::TomlVersion;
use tombi_document_tree::TryIntoDocumentTree;

fn find_workspace_cargo_toml(
    cargo_toml_path: &std::path::Path,
    toml_version: TomlVersion,
) -> Option<(std::path::PathBuf, tombi_document_tree::DocumentTree)> {
    let Some(mut current_dir) = cargo_toml_path.parent() else {
        return None;
    };

    while let Some(target_dir) = current_dir.parent() {
        current_dir = target_dir;
        let workspace_cargo_toml = current_dir.join("Cargo.toml");

        if workspace_cargo_toml.exists() {
            tracing::error!(?workspace_cargo_toml);
            let Some(toml_text) = std::fs::read_to_string(&workspace_cargo_toml).ok() else {
                continue;
            };

            let Some(root) =
                tombi_ast::Root::cast(tombi_parser::parse(&toml_text).into_syntax_node())
            else {
                continue;
            };

            let Ok(document_tree) = root.try_into_document_tree(toml_version) else {
                continue;
            };

            if document_tree.contains_key("workspace") {
                return Some((workspace_cargo_toml, document_tree));
            };
        }
    }

    None
}
