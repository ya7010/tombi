#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("An empty quoted key is allowed, but it is not recommended")]
    KeyEmpty,

    #[error("The key \"{key}\" is required")]
    KeyRequired { key: String },

    #[error("\"{key}\" is not allowed")]
    KeyNotAllowed { key: String },

    #[error("Expected a value of type {expected}, but found {actual}")]
    TypeMismatch {
        expected: schema_store::ValueType,
        actual: document_tree::ValueType,
    },

    #[error("The value must be one of [{}], but found {actual}", .expected.join(", "))]
    Eunmerate {
        expected: Vec<String>,
        actual: String,
    },

    #[error("The value must be > {maximum}, but found {actual}")]
    MaximumInteger { maximum: i64, actual: i64 },

    #[error("The value must be < {minimum}, but found {actual}")]
    MinimumInteger { minimum: i64, actual: i64 },

    #[error("The value must be ≥ {maximum}, but found {actual}")]
    ExclusiveMaximumInteger { maximum: i64, actual: i64 },

    #[error("The value must be ≤ {minimum}, but found {actual}")]
    ExclusiveMinimumInteger { minimum: i64, actual: i64 },

    #[error("The value {actual} is not a multiple of {multiple_of}")]
    MultipleOfInteger { multiple_of: i64, actual: i64 },

    #[error("The value must be > {maximum}, but found {actual}")]
    MaximumFloat { maximum: f64, actual: f64 },

    #[error("The value must be < {minimum}, but found {actual}")]
    MinimumFloat { minimum: f64, actual: f64 },

    #[error("The value must be ≥ {maximum}, but found {actual}")]
    ExclusiveMaximumFloat { maximum: f64, actual: f64 },

    #[error("The value must be ≤ {minimum}, but found {actual}")]
    ExclusiveMinimumFloat { minimum: f64, actual: f64 },

    #[error("The value {actual} is not a multiple of {multiple_of}")]
    MultipleOfFloat { multiple_of: f64, actual: f64 },

    #[error("The length must be ≤ {maximum}, but found {actual}")]
    MaximumLength { maximum: usize, actual: usize },

    #[error("The length must be ≥ {minimum}, but found {actual}")]
    MinimumLength { minimum: usize, actual: usize },

    #[error("The pattern \"{pattern}\" is invalid: {error}")]
    InvalidPattern { pattern: String, error: String },

    #[error("The value \"{actual}\" does not match the pattern \"{pattern}\"")]
    PatternMismatch { pattern: String, actual: String },

    #[error("The array must contain at least {min_items} items, but found {actual}")]
    MinItems { min_items: usize, actual: usize },

    #[error("The array must contain at most {max_items} items, but found {actual}")]
    MaxItems { max_items: usize, actual: usize },
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
