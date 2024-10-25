use crate::{server::backend::Backend, toml};
use ast::AstNode;
use text_position::TextPosition;
use tower_lsp::lsp_types::{
    lsif::Document, DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, MessageType,
    Position, Range, SymbolKind, Url,
};
pub async fn handle_document_symbol(
    backend: &Backend,
    params: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>, tower_lsp::jsonrpc::Error> {
    let source = toml::try_load(&params.text_document.uri)?;

    let p = parser::parse(&source);
    let symbol_infos = ast::Root::cast(p.into_syntax_node())
        .map(|root| root.into_symbols(&source))
        .unwrap_or_default();

    backend
        .client
        .log_message(MessageType::INFO, format!("Symbols: {symbol_infos:#?}"))
        .await;

    Ok(Some(DocumentSymbolResponse::Nested(symbol_infos)))
}

trait IntoSymbols {
    fn into_symbols(self, source: &str) -> Vec<DocumentSymbol>;
}

trait IntoSymbolsWithChildren {
    fn into_symbols_with_childen(
        self,
        source: &str,
        children: Option<Vec<DocumentSymbol>>,
    ) -> Vec<DocumentSymbol>;
}

trait IntoKeysSymbols {
    fn into_keys_symbols(
        self,
        source: &str,
        kind: tower_lsp::lsp_types::SymbolKind,
        children: Option<Vec<DocumentSymbol>>,
    ) -> Vec<DocumentSymbol>;
}

impl IntoSymbols for ast::Root {
    fn into_symbols(self, source: &str) -> Vec<DocumentSymbol> {
        self.items()
            .map(|item| item.into_symbols(source))
            .flatten()
            .collect()
    }
}

impl IntoSymbols for ast::RootItem {
    fn into_symbols(self, source: &str) -> Vec<DocumentSymbol> {
        match self {
            Self::KeyValue(kv) => kv.into_symbols(source),
            Self::Table(table) => table.into_symbols(source),
            _ => vec![],
        }
    }
}

impl IntoSymbols for ast::KeyValue {
    fn into_symbols(self, source: &str) -> Vec<DocumentSymbol> {
        if let Some(keys) = self.keys() {
            let children = self.value().map(|value| value.into_symbols(source));
            keys.into_keys_symbols(source, SymbolKind::VARIABLE, children)
        } else {
            vec![]
        }
    }
}

impl IntoKeysSymbols for ast::Keys {
    fn into_keys_symbols(
        self,
        source: &str,
        kind: tower_lsp::lsp_types::SymbolKind,
        children: Option<Vec<DocumentSymbol>>,
    ) -> Vec<DocumentSymbol> {
        let keys = self.keys().into_iter().collect::<Vec<_>>();
        let name = keys
            .iter()
            .map(|key| key.syntax().text().to_string())
            .collect::<Vec<_>>()
            .join(".");
        match (keys.first(), keys.last()) {
            (Some(first), Some(last)) => {
                let start_pos =
                    TextPosition::from_source(source, first.syntax().text_range().start());
                let end_pos = TextPosition::from_source(source, last.syntax().text_range().end());
                let range = Range {
                    start: Position {
                        line: start_pos.line(),
                        character: start_pos.column(),
                    },
                    end: Position {
                        line: end_pos.line(),
                        character: end_pos.column(),
                    },
                };
                #[allow(deprecated)]
                let symbol = DocumentSymbol {
                    name,
                    kind,
                    tags: None,
                    range,
                    selection_range: range,
                    children,
                    deprecated: None,
                    detail: None,
                };
                vec![symbol]
            }
            _ => vec![],
        }
    }
}

impl IntoSymbols for ast::Value {
    fn into_symbols(self, _source: &str) -> Vec<DocumentSymbol> {
        vec![]
    }
}

impl IntoSymbols for ast::Table {
    fn into_symbols(self, source: &str) -> Vec<DocumentSymbol> {
        self.header()
            .map(|header| header.into_keys_symbols(source, SymbolKind::STRUCT, None))
            .unwrap_or_default()
    }
}

// not document this function
#[allow(dead_code)]
fn debug_document_symbol() -> Vec<DocumentSymbol> {
    vec![
        #[allow(deprecated)]
        DocumentSymbol {
            name: "debug".to_string(),
            kind: tower_lsp::lsp_types::SymbolKind::VARIABLE,
            tags: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            children: None,
            deprecated: None,
            detail: None,
        },
    ]
}
