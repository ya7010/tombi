use tower_lsp::lsp_types::{DidChangeTextDocumentParams, DidOpenTextDocumentParams};

use crate::{
    converters::{
        encoding::{PositionEncoding, WideEncoding},
        from_lsp,
    },
    document::Document,
    server::backend::Backend,
    toml,
};

pub async fn handle_did_change(
    backend: &Backend,
    DidChangeTextDocumentParams {
        text_document,
        content_changes,
    }: DidChangeTextDocumentParams,
) {
    tracing::info!("handle_did_change");

    let uri = text_document.uri;
    let Some(document) = backend.documents.get(&uri) else {
        return;
    };

    let line_index = document.line_index();
    let mut source = document.source().to_string();

    for content_change in content_changes {
        if let Some(range) = content_change.range {
            let Ok(range) = from_lsp::text_range(
                line_index,
                range,
                PositionEncoding::Wide(WideEncoding::Utf16),
            ) else {
                continue;
            };

            source.replace_range(std::ops::Range::<usize>::from(range), &content_change.text);
        }
    }

    backend.documents.insert(uri, Document::new(source));
}
