use config::FormatOptions;
use dashmap::try_result::TryResult;
use itertools::Either;
use tower_lsp::lsp_types::{
    notification::ShowMessage, DocumentFormattingParams, MessageType, ShowMessageParams, TextEdit,
};

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_formatting(
    backend: &Backend,
    DocumentFormattingParams { text_document, .. }: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_formatting: {}", text_document.uri);

    let uri = &text_document.uri;
    let mut document_info = match backend.document_sources.try_get_mut(uri) {
        TryResult::Present(document_info) => document_info,
        TryResult::Absent => {
            tracing::warn!("document not found: {}", uri);
            return Ok(None);
        }
        TryResult::Locked => {
            tracing::warn!("document is locked: {}", uri);
            return Ok(None);
        }
    };

    match formatter::Formatter::try_new(
        backend.toml_version(),
        backend
            .config
            .format
            .as_ref()
            .unwrap_or(&FormatOptions::default()),
        Some(Either::Left(&text_document.uri)),
        &backend.schema_store,
    )
    .await
    {
        Ok(formatter) => match formatter.format(&document_info.source).await {
            Ok(new_text) => {
                if new_text != document_info.source {
                    document_info.source = new_text.clone();

                    return Ok(Some(vec![TextEdit {
                        range: text::Range::new(text::Position::MIN, text::Position::MAX).into(),
                        new_text,
                    }]));
                } else {
                    tracing::info!("no change");
                }
            }
            Err(_) => {
                tracing::error!("failed to format");
            }
        },
        Err(err) => {
            tracing::error!("{err}");

            backend
                .client
                .send_notification::<ShowMessage>(ShowMessageParams {
                    typ: MessageType::ERROR,
                    message: err.to_string(),
                })
                .await;
        }
    }

    Ok(None)
}
