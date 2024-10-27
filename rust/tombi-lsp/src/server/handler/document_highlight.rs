use crate::{server::backend::Backend, toml};
use ast::{algo::find_node_at_offset, AstNode};
use text_position::TextPosition;
use text_size::TextSize;
use tower_lsp::lsp_types::{
    DocumentHighlight, DocumentHighlightKind, DocumentHighlightParams, MessageType, Position,
    Range, TextDocumentPositionParams,
};

pub async fn handle_document_highlight(
    backend: &Backend,
    DocumentHighlightParams {
        text_document_position_params:
            TextDocumentPositionParams {
                text_document,
                position,
            },
        ..
    }: DocumentHighlightParams,
) -> Result<Option<Vec<DocumentHighlight>>, tower_lsp::jsonrpc::Error> {
    let source = toml::try_load(&text_document.uri)?;

    let (highlights, message) = {
        let mut highlights = Vec::new();

        let p = parser::parse(&source);
        let Some(ast) = ast::Root::cast(p.into_syntax_node()) else {
            return Ok(None);
        };
        let offset = TextSize::new(TextPosition::from(&source, position).offset() as u32);
        if let Some(node) = find_node_at_offset::<ast::Table>(&ast.syntax(), offset) {
            if let Some(header) = node.header() {
                if find_node_at_offset::<ast::Keys>(header.syntax(), offset).is_some() {
                    let start =
                        TextPosition::from_source(&source, node.syntax().text_range().start());
                    let end = TextPosition::from_source(&source, node.syntax().text_range().end());
                    let range = Range {
                        start: Position {
                            line: start.line(),
                            character: start.column(),
                        },
                        end: Position {
                            line: end.line(),
                            character: end.column(),
                        },
                    };
                    highlights.push(DocumentHighlight {
                        range,
                        kind: Some(DocumentHighlightKind::READ),
                    });
                }
            }
        };

        (highlights, format!("Ast Hilight: {:#?}", ast))
    };
    backend.client.log_message(MessageType::INFO, message).await;

    backend
        .client
        .log_message(MessageType::INFO, format!("Highlights"))
        .await;

    Ok(Some(highlights))
}
