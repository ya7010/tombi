use config::LintOptions;
use itertools::Either;
use tower_lsp::lsp_types::{
    notification::ShowMessage, DocumentDiagnosticParams, DocumentDiagnosticReport,
    DocumentDiagnosticReportResult, FullDocumentDiagnosticReport, MessageType,
    RelatedFullDocumentDiagnosticReport, ShowMessageParams,
};

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_diagnostic(
    backend: &Backend,
    DocumentDiagnosticParams { text_document, .. }: DocumentDiagnosticParams,
) -> Result<DocumentDiagnosticReportResult, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_diagnostic");

    let config = backend.config().await;

    if !config
        .server
        .and_then(|server| server.diagnostics)
        .and_then(|diagnostics| diagnostics.enabled)
        .unwrap_or_default()
        .value()
    {
        tracing::debug!("`server.diagnostics.enabled` is false");
        return Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(Default::default()),
        ));
    }

    let document_sources = backend.document_sources.read().await;
    let diagnostics = match document_sources.get(&text_document.uri) {
        Some(document) => {
            match linter::Linter::try_new(
                backend.toml_version().await.unwrap_or_default(),
                backend
                    .config()
                    .await
                    .lint
                    .as_ref()
                    .unwrap_or(&LintOptions::default()),
                Some(Either::Left(&text_document.uri)),
                &backend.schema_store,
            )
            .await
            {
                Ok(linter) => linter.lint(&document.source).await.map_or_else(
                    |diagnostics| diagnostics.into_iter().map(Into::into).collect(),
                    |_| vec![],
                ),
                Err(err) => {
                    tracing::error!("{err}");

                    backend
                        .client
                        .send_notification::<ShowMessage>(ShowMessageParams {
                            typ: MessageType::ERROR,
                            message: err.to_string(),
                        })
                        .await;
                    vec![]
                }
            }
        }
        None => vec![],
    };

    Ok(DocumentDiagnosticReportResult::Report(
        DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
            full_document_diagnostic_report: FullDocumentDiagnosticReport {
                items: diagnostics,
                ..Default::default()
            },
            ..Default::default()
        }),
    ))
}
