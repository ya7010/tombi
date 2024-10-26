use crate::{server::backend::Backend, toml};
use ast::AstNode;
use document::{Parse, Value};
use text_position::TextPosition;
use text_size::TextRange;
use tower_lsp::lsp_types::{
    lsif::Document, DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, MessageType,
    Position, Range, SymbolKind,
};

pub async fn handle_document_symbol(
    backend: &Backend,
    params: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>, tower_lsp::jsonrpc::Error> {
    let source = toml::try_load(&params.text_document.uri)?;

    let p = parser::parse(&source);
    let Some(root) = ast::Root::cast(p.into_syntax_node()) else {
        return Ok(None);
    };

    let pp = root.parse(&source);
    backend
        .client
        .log_message(MessageType::INFO, format!("Document: {:#?}", pp.document()))
        .await;
    let symbols = create_symbols(pp.document());

    backend
        .client
        .log_message(MessageType::INFO, format!("Symbols: {symbols:#?}"))
        .await;

    Ok(Some(DocumentSymbolResponse::Nested(symbols)))
}

fn create_symbols(document: &document::Table) -> Vec<DocumentSymbol> {
    let mut symbols: Vec<DocumentSymbol> = vec![];

    for (key, entry) in document.entries() {
        symbols_for_value(key.to_string(), None, entry, &mut symbols);
    }

    symbols
}

#[allow(deprecated)]
fn symbols_for_value(
    name: String,
    key_range: Option<document::Range>,
    node: &document::Value,
    symbols: &mut Vec<DocumentSymbol>,
) {
    let own_range = node.range();
    let range = if let Some(key_r) = key_range {
        key_r.merge(&own_range)
    } else {
        own_range
    };

    let selection_range = key_range.map_or(own_range, |range| range);

    match node {
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
        Value::DateTime(_) => {
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
        Value::Date(_) => {
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
        Value::Time(_) => {
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
            for (index, element) in array.elements().iter().enumerate() {
                symbols_for_value(index.to_string(), Some(range), element, &mut children);
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
            for (key, entry) in table.entries() {
                symbols_for_value(key.to_string(), Some(range), entry, &mut children);
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
