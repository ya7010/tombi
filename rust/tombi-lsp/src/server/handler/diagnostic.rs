use text::Position;
use tower_lsp::lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, Range, RelatedFullDocumentDiagnosticReport,
};

use crate::toml;

pub async fn handle_diagnostic(
    DocumentDiagnosticParams { text_document, .. }: DocumentDiagnosticParams,
) -> Result<DocumentDiagnosticReportResult, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_diagnostic");

    let source = toml::try_load(&text_document.uri)?;

    let items = if let Err(errors) = formatter::format(&source) {
        errors
            .iter()
            .map(|error| {
                let range = error.range();
                let range = Range::new(
                    text::Position::from_source(&source, range.start()).into(),
                    text::Position::from_source(&source, range.end()).into(),
                );
                tower_lsp::lsp_types::Diagnostic {
                    range,
                    severity: Some(tower_lsp::lsp_types::DiagnosticSeverity::ERROR),
                    message: error.message().to_string(),
                    ..Default::default()
                }
            })
            .collect()
    } else {
        vec![]
    };

    Ok(DocumentDiagnosticReportResult::Report(
        DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
            full_document_diagnostic_report: FullDocumentDiagnosticReport {
                items,
                ..Default::default()
            },
            ..Default::default()
        }),
    ))
}
