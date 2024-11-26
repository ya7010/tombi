use crate::backend::Backend;
use ast::AstNode;
use document::{Node, Parse};
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

    let Some(ast) = ast::Root::cast(
        parser::parse(&document.source, syntax::TomlVersion::default()).into_syntax_node(),
    ) else {
        return Ok(None);
    };

    let (document, _) = ast.parse(&document.source).into();

    let symbols = create_symbols(&document);

    tracing::debug!("DocumentSymbols: {symbols:#?}");

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
    node: &document::Node,
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
        Node::Boolean(_) => {
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
        Node::Integer(_) => {
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
        Node::Float(_) => {
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
        Node::String(_) => {
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
        Node::DateTime(_) => {
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
        Node::Date(_) => {
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
        Node::Time(_) => {
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
        Node::Array(array) => {
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
        Node::Table(table) => {
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
