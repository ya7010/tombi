#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Config(#[from] config::Error),

    #[error(transparent)]
    SchemaStore(#[from] schema_store::Error),

    #[error(transparent)]
    DocumentDeserialize(#[from] document::de::Error),

    #[error("{}", .0.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))]
    Parser(Vec<parser::Error>),

    #[error("{}", .0.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))]
    DocumentTree(Vec<document_tree::Error>),
}
