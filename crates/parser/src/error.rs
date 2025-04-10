use config::TomlVersion;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ErrorKind {
    // lexer::ErrorKind translation
    #[error("invalid key")]
    InvalidKey,

    #[error("invalid basic string")]
    InvalidBasicString,

    #[error("invalid literal string")]
    InvalidLiteralString,

    #[error("invalid multi-line basic strings")]
    InvalidMultilineBasicString,

    #[error("invalid multi-line literal strings")]
    InvalidMultilineLiteralString,

    #[error("invalid number")]
    InvalidNumber,

    #[error("invalid offset date-time")]
    InvalidOffsetDateTime,

    #[error("invalid local date-time")]
    InvalidLocalDateTime,

    #[error("invalid local date")]
    InvalidLocalDate,

    #[error("invalid local time")]
    InvalidLocalTime,

    #[error("invalid line break")]
    InvalidLineBreak,

    #[error("invalid token")]
    InvalidToken,

    // Grammar error
    #[error("unknown line")]
    UnknownLine,

    #[error("expected key")]
    ExpectedKey,

    #[error("expected value")]
    ExpectedValue,

    #[error("expected '='")]
    ExpectedEqual,

    #[error("expected ','")]
    ExpectedComma,

    #[error("expected ']'")]
    ExpectedBracketEnd,

    #[error("expected ']]'")]
    ExpectedDoubleBracketEnd,

    #[error("expected '}}'")]
    ExpectedBraceEnd,

    #[error("expected line break")]
    ExpectedLineBreak,

    #[error("forbidden last period in keys")]
    ForbiddenKeysLastPeriod,

    #[error("inline table must be single line in TOML v1.0.0 or earlier")]
    InlineTableMustSingleLine,

    #[error("trailing comma in inline table not allowed in TOML v1.0.0 or earlier")]
    ForbiddenInlineTableLastComma,
}

#[derive(thiserror::Error, Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    range: text::Range,
}

impl Error {
    pub fn new(kind: ErrorKind, range: text::Range) -> Self {
        Self { kind, range }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn to_message(&self) -> String {
        self.kind.to_string()
    }

    pub fn range(&self) -> text::Range {
        self.range
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} in {}..{}",
            self.kind,
            self.range.start(),
            self.range.end()
        )
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.range == other.range
    }
}

impl Eq for Error {}

impl From<lexer::Error> for Error {
    fn from(error: lexer::Error) -> Self {
        let kind = match error.kind() {
            lexer::ErrorKind::InvalidKey => ErrorKind::InvalidKey,
            lexer::ErrorKind::InvalidBasicString => ErrorKind::InvalidBasicString,
            lexer::ErrorKind::InvalidLiteralString => ErrorKind::InvalidLiteralString,
            lexer::ErrorKind::InvalidMultilineBasicString => ErrorKind::InvalidMultilineBasicString,
            lexer::ErrorKind::InvalidMultilineLiteralString => {
                ErrorKind::InvalidMultilineLiteralString
            }
            lexer::ErrorKind::InvalidNumber => ErrorKind::InvalidNumber,
            lexer::ErrorKind::InvalidOffsetDateTime => ErrorKind::InvalidOffsetDateTime,
            lexer::ErrorKind::InvalidLocalDateTime => ErrorKind::InvalidLocalDateTime,
            lexer::ErrorKind::InvalidLocalDate => ErrorKind::InvalidLocalDate,
            lexer::ErrorKind::InvalidLocalTime => ErrorKind::InvalidLocalTime,
            lexer::ErrorKind::InvalidLineBreak => ErrorKind::InvalidLineBreak,
            lexer::ErrorKind::InvalidToken => ErrorKind::InvalidToken,
        };

        Self::new(kind, error.range())
    }
}

#[cfg(feature = "diagnostic")]
impl diagnostic::SetDiagnostics for Error {
    fn set_diagnostics(&self, diagnostics: &mut Vec<diagnostic::Diagnostic>) {
        diagnostics.push(diagnostic::Diagnostic::new_error(
            self.to_message(),
            self.range(),
        ));
    }
}

/// A wrapper type for errors that includes information about whether the error depends on the version of TOML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TomlVersionedError {
    /// Errors that do not depend on the version of TOML
    Common(Error),
    /// Error when not compatible with new syntax
    NewSyntax {
        error: Error,
        minimum_version: TomlVersion,
    },
}

impl TomlVersionedError {
    pub fn error(&self) -> &Error {
        match self {
            Self::Common(error) => error,
            Self::NewSyntax { error, .. } => error,
        }
    }

    pub fn into_error(self) -> Error {
        match self {
            Self::Common(error) => error,
            Self::NewSyntax { error, .. } => error,
        }
    }

    pub fn is_compatible_with(&self, toml_version: TomlVersion) -> bool {
        match self {
            Self::Common(_) => true,
            Self::NewSyntax {
                minimum_version, ..
            } => toml_version < *minimum_version,
        }
    }
}

impl From<Error> for TomlVersionedError {
    fn from(error: Error) -> Self {
        Self::Common(error)
    }
}

impl From<lexer::Error> for TomlVersionedError {
    fn from(error: lexer::Error) -> Self {
        Self::Common(error.into())
    }
}
