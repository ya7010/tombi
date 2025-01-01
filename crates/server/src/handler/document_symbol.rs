use crate::backend::Backend;
use document_tree::TryIntoDocumentTree;
use tower_lsp::lsp_types::{
    DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind,
};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_document_symbol(
    backend: &Backend,
    DocumentSymbolParams { text_document, .. }: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_document_symbol");
    let toml_version = backend.toml_version().await.unwrap_or_default();

    let Some(root) = backend.get_ast(&text_document.uri, toml_version) else {
        return Ok(None);
    };

    let Ok(root) = root.try_into_document_tree(toml_version) else {
        return Ok(None);
    };

    let symbols = create_symbols(&root);

    tracing::trace!("DocumentSymbols: {symbols:#?}");

    Ok(Some(DocumentSymbolResponse::Nested(symbols)))
}

fn create_symbols(root: &document_tree::Root) -> Vec<DocumentSymbol> {
    let mut symbols: Vec<DocumentSymbol> = vec![];

    for (key, value) in root.key_values() {
        symbols_for_value(key.to_string(), value, None, &mut symbols);
    }

    symbols
}

#[allow(deprecated)]
fn symbols_for_value(
    name: String,
    value: &document_tree::Value,
    parent_key_range: Option<text::Range>,
    symbols: &mut Vec<DocumentSymbol>,
) {
    use document_tree::Value::*;

    let value_range = value.symbol_range();
    let range = if let Some(parent_key_range) = parent_key_range {
        parent_key_range + value_range
    } else {
        value_range
    };

    let selection_range = range;

    match value {
        Boolean { .. } => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::BOOLEAN,
                range: range.into(),
                selection_range: selection_range.into(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        Integer { .. } | Float { .. } => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::NUMBER,
                range: range.into(),
                selection_range: selection_range.into(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        String { .. } => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::STRING,
                range: range.into(),
                selection_range: selection_range.into(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        OffsetDateTime { .. } | LocalDateTime { .. } | LocalDate { .. } | LocalTime { .. } => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::STRING,
                range: range.into(),
                selection_range: selection_range.into(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        Array(array) => {
            let mut children = vec![];
            for (index, value) in array.values().iter().enumerate() {
                symbols_for_value(
                    format!("[{index}]"),
                    value,
                    Some(value.symbol_range()),
                    &mut children,
                );
            }

            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::ARRAY,
                range: range.into(),
                selection_range: selection_range.into(),
                children: Some(children),
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        Table(table) => {
            let mut children = vec![];
            for (key, value) in table.key_values() {
                symbols_for_value(key.to_string(), value, Some(key.range()), &mut children);
            }

            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::OBJECT,
                range: range.into(),
                selection_range: selection_range.into(),
                children: Some(children),
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
    }
}
