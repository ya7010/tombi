#[derive(thiserror::Error, Debug)]
pub enum WarningKind {
    #[error("This value is deprecated")]
    Deprecated,
}

#[derive(Debug)]
pub struct Warning {
    pub kind: WarningKind,
    pub range: text::Range,
}

impl diagnostic::SetDiagnostics for Warning {
    fn set_diagnostics(&self, diagnostics: &mut Vec<diagnostic::Diagnostic>) {
        diagnostics.push(diagnostic::Diagnostic::new_warning(
            self.kind.to_string(),
            self.range,
        ))
    }
}
