mod document;
mod hint;
mod table;
mod value;

pub use hint::CompletionHint;
use schema_store::{Accessor, SchemaDefinitions};
use tower_lsp::lsp_types::{CompletionItem, MarkupContent, MarkupKind};

pub trait FindCompletionItems {
    fn find_completion_items(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (Vec<CompletionItem>, Vec<schema_store::Error>);
}

pub trait Completion {
    fn title(&self) -> Option<&str>;

    fn description(&self) -> Option<&str>;

    fn detail(&self) -> Option<String> {
        self.title().map(ToOwned::to_owned)
    }

    fn documentation(&self) -> Option<tower_lsp::lsp_types::Documentation> {
        self.description().map(|description| {
            tower_lsp::lsp_types::Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: description.to_string(),
            })
        })
    }
}
