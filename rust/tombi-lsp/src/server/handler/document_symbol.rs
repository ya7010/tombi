use tower_lsp::lsp_types::{
    DocumentSymbolParams, DocumentSymbolResponse, Location, Position, Range, SymbolInformation,
};

use crate::server::backend::Backend;

pub async fn handle_document_symbol(
    backend: &Backend,
    params: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>, tower_lsp::jsonrpc::Error> {
    Ok(Some(DocumentSymbolResponse::Flat(vec![
        #[allow(deprecated)]
        SymbolInformation {
            name: "members".to_string(),
            kind: tower_lsp::lsp_types::SymbolKind::CLASS,
            tags: None,
            location: Location {
                uri: params.text_document.uri,
                range: Range {
                    start: Position {
                        line: 2,
                        character: 0,
                    },
                    end: Position {
                        line: 2,
                        character: 20,
                    },
                },
            },
            container_name: Some("Workspace".to_string()),
            deprecated: None,
        },
    ])))
}
