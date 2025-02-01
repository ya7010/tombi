#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompletionKind {
    Boolean,
    Integer,
    Float,
    String,
    OffsetDateTime,
    LocalDateTime,
    LocalDate,
    LocalTime,
    Array,
    Table,
    Property,
}

impl From<CompletionKind> for tower_lsp::lsp_types::CompletionItemKind {
    fn from(kind: CompletionKind) -> Self {
        match kind {
            CompletionKind::Boolean => tower_lsp::lsp_types::CompletionItemKind::ENUM_MEMBER,
            CompletionKind::Integer => tower_lsp::lsp_types::CompletionItemKind::VALUE,
            CompletionKind::Float => tower_lsp::lsp_types::CompletionItemKind::VALUE,
            CompletionKind::String => tower_lsp::lsp_types::CompletionItemKind::TEXT,
            CompletionKind::OffsetDateTime => tower_lsp::lsp_types::CompletionItemKind::EVENT,
            CompletionKind::LocalDateTime => tower_lsp::lsp_types::CompletionItemKind::EVENT,
            CompletionKind::LocalDate => tower_lsp::lsp_types::CompletionItemKind::EVENT,
            CompletionKind::LocalTime => tower_lsp::lsp_types::CompletionItemKind::EVENT,
            CompletionKind::Array => tower_lsp::lsp_types::CompletionItemKind::VALUE,
            CompletionKind::Table => tower_lsp::lsp_types::CompletionItemKind::STRUCT,
            CompletionKind::Property => tower_lsp::lsp_types::CompletionItemKind::PROPERTY,
        }
    }
}
