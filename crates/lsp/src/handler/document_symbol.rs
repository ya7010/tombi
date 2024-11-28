use crate::backend::Backend;
use ast::AstNode;
use config::TomlVersion;
use document::Value;
use tower_lsp::lsp_types::{
    DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind,
};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_document_symbol(
    backend: &Backend,
    DocumentSymbolParams { text_document, .. }: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_document_symbol");

    let Some(document) = backend.documents.get(&text_document.uri) else {
        return Ok(None);
    };

    let Some(ast) =
        ast::Root::cast(parser::parse(&document.source, TomlVersion::default()).into_syntax_node())
    else {
        return Ok(None);
    };

    let Ok(document) = ast.try_into() else {
        return Ok(None);
    };

    let symbols = create_symbols(&document);

    tracing::debug!("DocumentSymbols: {symbols:#?}");

    Ok(Some(DocumentSymbolResponse::Nested(symbols)))
}

fn create_symbols(document: &document::Document) -> Vec<DocumentSymbol> {
    let mut symbols: Vec<DocumentSymbol> = vec![];

    for (key, value) in document.key_values() {
        symbols_for_value(key.to_string(), Some(key.range()), value, &mut symbols);
    }

    symbols
}

#[allow(deprecated)]
fn symbols_for_value(
    name: String,
    key_range: Option<text::Range>,
    value: &document::Value,
    symbols: &mut Vec<DocumentSymbol>,
) {
    let value_range = value.range();
    let range = if let Some(key_range) = key_range {
        key_range + value_range
    } else {
        value_range
    };

    let selection_range = key_range.map_or(value_range, |range| range);

    match value {
        Value::Boolean(_) => {
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
        Value::Integer(_) => {
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
        Value::Float(_) => {
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
        Value::String(_) => {
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
        Value::OffsetDateTime(_)
        | Value::LocalDateTime(_)
        | Value::LocalDate(_)
        | Value::LocalTime(_) => {
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
        Value::Array(array) => {
            let mut children = vec![];
            for (index, value) in array.values().iter().enumerate() {
                symbols_for_value(index.to_string(), Some(range), value, &mut children);
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
        Value::Table(table) => {
            let mut children = vec![];
            for (key, value) in table.key_values() {
                symbols_for_value(key.to_string(), Some(range), value, &mut children);
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
