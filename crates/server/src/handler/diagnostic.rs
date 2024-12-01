use tower_lsp::lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};

use crate::backend::Backend;
use crate::document::Document;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_diagnostic(
    backend: &Backend,
    DocumentDiagnosticParams { text_document, .. }: DocumentDiagnosticParams,
) -> Result<DocumentDiagnosticReportResult, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_diagnostic");

    let diagnostics = get_diagnostics(backend.documents.get(&text_document.uri).as_deref()).await;

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

pub async fn get_diagnostics(document: Option<&Document>) -> Vec<tower_lsp::lsp_types::Diagnostic> {
    let Some(document) = document else {
        return vec![];
    };

    if let Err(diagnostics) = linter::lint(&document.source) {
        diagnostics
            .into_iter()
            .map(|diagnostic| tower_lsp::lsp_types::Diagnostic {
                range: tower_lsp::lsp_types::Range::from(diagnostic.range()),
                severity: Some(match diagnostic.level() {
                    diagnostic::Level::Warning => tower_lsp::lsp_types::DiagnosticSeverity::WARNING,
                    diagnostic::Level::Error => tower_lsp::lsp_types::DiagnosticSeverity::ERROR,
                }),
                message: diagnostic.message().to_string(),
                ..Default::default()
            })
            .collect()
    } else {
        vec![]
    }
}
