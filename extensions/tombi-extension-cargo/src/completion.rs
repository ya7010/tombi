use tombi_config::TomlVersion;
use tombi_extension::CompletionContent;
use tower_lsp::lsp_types::TextDocumentIdentifier;

pub async fn completion(
    _text_document: &TextDocumentIdentifier,
    _document_tree: &tombi_document_tree::DocumentTree,
    _accessors: &[tombi_schema_store::Accessor],
    _toml_version: TomlVersion,
) -> Result<Option<Vec<CompletionContent>>, tower_lsp::jsonrpc::Error> {
    Ok(None)
}
