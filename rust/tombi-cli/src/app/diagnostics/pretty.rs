use std::io::Write;

use tombi_diagnostic::{Print, printer::Pretty};

use super::{FileReport, format_reporter::FormatReporter};

/// Writes human-readable diagnostics as soon as each file is recorded.
pub(super) struct PrettyReporter {
    printer: Pretty,
}

impl PrettyReporter {
    pub(super) fn new(use_ansi_color: bool) -> Self {
        Self {
            printer: Pretty { use_ansi_color },
        }
    }
}

impl FormatReporter for PrettyReporter {
    fn record(&mut self, file: FileReport, writer: &mut dyn Write) -> std::io::Result<()> {
        let Some(line_index) = &file.line_index else {
            return Ok(());
        };
        for diagnostic in &file.diagnostics {
            diagnostic
                .located(line_index, tombi_text::EncodingKind::GraphemeCluster)
                .print(&self.printer, writer)?;
        }
        Ok(())
    }

    fn record_runtime_error(&mut self) {}

    fn finish(&mut self, _writer: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }

    fn reports_file_problems(&self) -> bool {
        false
    }
}
