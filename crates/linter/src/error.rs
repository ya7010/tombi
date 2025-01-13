#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("An empty quoted key is allowed, but it is not recommended")]
    KeyEmpty,

    #[error("\"{key}\" is not allowed")]
    KeyNotAllowed { key: String },
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub range: text::Range,
}

impl diagnostic::SetDiagnostics for Error {
    fn set_diagnostic(&self, diagnostics: &mut Vec<diagnostic::Diagnostic>) {
        diagnostics.push(diagnostic::Diagnostic::new_error(
            self.kind.to_string(),
            self.range,
        ))
    }
}
