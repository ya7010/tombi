use dashmap::try_result::TryResult;
use tower_lsp::lsp_types::DidChangeTextDocumentParams;

use crate::{
    converters::{
        encoding::{PositionEncoding, WideEncoding},
        from_lsp,
    },
    server::backend::Backend,
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
    let mut document = match backend.documents.try_get_mut(&uri) {
        TryResult::Present(document) => document,
        TryResult::Absent => {
            tracing::warn!("document not found: {}", uri);
            return;
        }
        TryResult::Locked => {
            tracing::warn!("document is locked: {}", uri);
            return;
        }
    };

    tracing::info!("content_changes: {content_changes:#?}");
    for content_change in content_changes {
        if let Some(range) = content_change.range {
            let Ok(range) = from_lsp::text_range(
                &document.line_index,
                range,
                PositionEncoding::Wide(WideEncoding::Utf16),
            ) else {
                continue;
            };

            document
                .source
                .replace_range(std::ops::Range::<usize>::from(range), &content_change.text);
        }
    }
}
