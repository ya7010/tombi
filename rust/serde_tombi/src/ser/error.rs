#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Config(#[from] config::Error),

    #[error(transparent)]
    SchemaStore(#[from] schema_store::Error),

    #[error("Document root must be a Table, got {0}")]
    RootMustBeTable(document::ValueKind),

    #[error("Key must be a String, got {1} for {0}")]
    KeyMustBeString(schema_store::Accessors, document::ValueKind),

    #[error("Key is required for {0}")]
    KeyRequired(schema_store::Accessors),

    #[error("Value is required for {0}")]
    ValueRequired(schema_store::Accessors),

    #[error("{error} for {accessors}")]
    DateTimeParseFailed {
        accessors: schema_store::Accessors,
        error: date_time::parse::Error,
    },

    #[error("TOML must be UTF-8 encoded")]
    TomlMustBeUtf8,

    #[error("{0} required")]
    ArrayValueRequired(schema_store::Accessors),

    #[error("TOML does not support unit values")]
    SerializeUnit,

    #[error("TOML does not support unit structs")]
    SerializeUnitStruct,

    #[error("{0}")]
    Serde(std::string::String),
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Serde(msg.to_string())
    }
}
