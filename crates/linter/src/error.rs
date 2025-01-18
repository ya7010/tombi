use schema_store::ValueType;

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("An empty quoted key is allowed, but it is not recommended")]
    KeyEmpty,

    #[error("\"{key}\" is not allowed")]
    KeyNotAllowed { key: String },

    #[error("Expected a value of type {expected}, but found {actual}")]
    TypeMismatch {
        expected: ValueType,
        actual: ValueType,
    },

    #[error("Expected one of {expected}, but found {actual}")]
    InvalidValue { expected: String, actual: String },

    #[error("The value {actual} exceeds the maximum value of {maximum}")]
    MaximumInteger { maximum: i64, actual: i64 },

    #[error("The value {actual} is less than the minimum value of {minimum}")]
    MinimumInteger { minimum: i64, actual: i64 },

    #[error("The value {actual} exceeds the exclusive maximum value of {maximum}")]
    ExclusiveMaximumInteger { maximum: i64, actual: i64 },

    #[error("The value {actual} is less than the exclusive minimum value of {minimum}")]
    ExclusiveMinimumInteger { minimum: i64, actual: i64 },

    #[error("The value {actual} is not a multiple of {multiple_of}")]
    MultipleOfInteger { multiple_of: i64, actual: i64 },
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
