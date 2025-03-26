use config::LintOptions;
use itertools::Either;
use tower_lsp::lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};

use crate::backend::Backend;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_diagnostic(
    backend: &Backend,
    params: DocumentDiagnosticParams,
) -> Result<DocumentDiagnosticReportResult, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_diagnostic");
    tracing::trace!(?params);

    let DocumentDiagnosticParams { text_document, .. } = params;

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
        Some(document) => linter::Linter::new(
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
        .lint(&document.source)
        .await
        .map_or_else(
            |diagnostics| diagnostics.into_iter().map(Into::into).collect(),
            |_| vec![],
        ),
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
