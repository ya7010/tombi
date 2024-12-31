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
    DocumentDiagnosticParams { text_document, .. }: DocumentDiagnosticParams,
) -> Result<DocumentDiagnosticReportResult, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_diagnostic");

    let diagnostics = match backend.document_sources.get(&text_document.uri).as_deref() {
        Some(document) => {
            match linter::Linter::try_new(
                backend.toml_version(),
                backend
                    .config
                    .lint
                    .as_ref()
                    .unwrap_or(&LintOptions::default()),
                Some(Either::Left(&text_document.uri)),
                &backend.schema_store,
            )
            .await
            {
                Ok(linter) => linter.lint(&document.source).await.map_or_else(
                    |diagnostics| {
                        diagnostics
                            .into_iter()
                            .map(|diagnostic| tower_lsp::lsp_types::Diagnostic {
                                range: diagnostic.range().into(),
                                severity: Some(match diagnostic.level() {
                                    diagnostic::Level::WARNING => {
                                        tower_lsp::lsp_types::DiagnosticSeverity::WARNING
                                    }
                                    diagnostic::Level::ERROR => {
                                        tower_lsp::lsp_types::DiagnosticSeverity::ERROR
                                    }
                                }),
                                message: diagnostic.message().to_string(),
                                ..Default::default()
                            })
                            .collect()
                    },
                    |_| vec![],
                ),
                Err(_) => return Err(tower_lsp::jsonrpc::Error::internal_error()),
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
