use tombi_schema_store::{Accessor, AccessorContext};
use tower_lsp::lsp_types::CodeActionOrCommand;

pub fn code_action(
    _document_tree: &tombi_document_tree::DocumentTree,
    _accessors: &[Accessor],
    _contexts: &[AccessorContext],
) -> Result<Option<Vec<CodeActionOrCommand>>, tower_lsp::jsonrpc::Error> {
    let actions = Vec::new();

    // Check for other code actions based on the document tree
    // (e.g., formatting, linting, etc.)

    Ok(if actions.is_empty() {
        None
    } else {
        Some(actions)
    })
}
