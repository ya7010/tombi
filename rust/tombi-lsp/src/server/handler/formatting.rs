use text::TextSize;
use tower_lsp::lsp_types::{DocumentFormattingParams, Range, TextEdit};

use crate::{document::Document, server::backend::Backend};

pub async fn handle_formatting(
    backend: &Backend,
    DocumentFormattingParams { text_document, .. }: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_formatting");

    let uri = &text_document.uri;
    let Some(document) = backend.documents.get(&uri) else {
        return Ok(None);
    };
    let source = document.source();

    if let Ok(new_text) = formatter::format(&source) {
        if new_text != source {
            backend
                .documents
                .insert(text_document.uri, Document::new(new_text.clone()));
            return Ok(Some(vec![TextEdit {
                range: Range::new(
                    text::Position::new(0, 0).into(),
                    text::Position::from_source(&source, TextSize::new(source.len() as u32)).into(),
                ),
                new_text,
            }]));
        }
    }

    Ok(None)
}
