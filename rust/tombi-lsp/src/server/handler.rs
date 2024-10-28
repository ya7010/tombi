mod diagnostic;
mod did_change_configuration;
mod document_symbol;
mod formatting;
mod hover;
mod initialize;
mod semantic_tokens_full;
mod shutdown;

pub use diagnostic::handle_diagnostic;
pub use did_change_configuration::handle_did_change_configuration;
pub use document_symbol::handle_document_symbol;
pub use formatting::handle_formatting;
pub use hover::handle_hover;
pub use initialize::handle_initialize;
pub use semantic_tokens_full::handle_semantic_tokens_full;
pub use shutdown::handle_shutdown;
