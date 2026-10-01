use std::sync::Arc;

use tombi_diagnostic::SetDiagnostics;
use tombi_text::{EncodingKind, LineIndex};

use tombi_document_tree_syntax::IntoDocumentTreeAndErrors;

#[derive(Debug, Clone)]
pub struct DocumentSource {
    /// The parse result of the text, which owns the text and its line index.
    ///
    /// The linter and the formatter reuse it, so the text is parsed and indexed only once.
    parsed: tombi_parser::ParseResult,

    /// The line index of the text, built while parsing it.
    line_index: Arc<LineIndex>,

    /// The column unit of the LSP client.
    encoding_kind: EncodingKind,

    /// The version of the document.
    ///
    /// If the file has never been opened in the editor, None will be entered.
    pub version: Option<i32>,

    pub toml_version: tombi_config::TomlVersion,

    /// Parsed AST (always exists, even with errors)
    ast: Arc<tombi_ast_syntax::Root>,

    /// AST generation errors (empty if no errors)
    ast_errors: Vec<tombi_diagnostic::Diagnostic>,

    /// Parsed DocumentTree (always exists)
    document_tree: Arc<tombi_document_tree_syntax::DocumentTree>,

    /// DocumentTree generation errors (empty if no errors)
    document_tree_errors: Vec<tombi_diagnostic::Diagnostic>,
}

impl DocumentSource {
    pub fn new(
        parsed: tombi_parser::ParseResult,
        version: Option<i32>,
        toml_version: tombi_config::TomlVersion,
        encoding_kind: EncodingKind,
    ) -> Self {
        let ast = parsed.root();
        let mut ast_errors = Vec::with_capacity(parsed.errors.len());
        for error in parsed.errors.iter().cloned() {
            error.set_diagnostics(&mut ast_errors);
        }
        let (document_tree, document_tree_errors) = build_document_tree(&ast, toml_version);

        Self {
            line_index: Arc::clone(parsed.line_index()),
            parsed,
            encoding_kind,
            version,
            toml_version,
            ast: Arc::new(ast),
            ast_errors,
            document_tree,
            document_tree_errors,
        }
    }

    pub fn text(&self) -> &str {
        self.line_index.text()
    }

    /// The parse result of the text, to lint or format it without parsing it again.
    pub fn parsed(&self) -> &tombi_parser::ParseResult {
        &self.parsed
    }

    pub fn set_text(&mut self, text: &str, toml_version: tombi_config::TomlVersion) {
        *self = Self::new(
            tombi_parser::parse(text),
            self.version,
            toml_version,
            self.encoding_kind,
        );
    }

    /// Rebuilds only the document tree for `toml_version`, reusing the parsed AST.
    pub fn set_toml_version(&mut self, toml_version: tombi_config::TomlVersion) {
        self.toml_version = toml_version;
        (self.document_tree, self.document_tree_errors) =
            build_document_tree(&self.ast, toml_version);
    }

    pub fn line_index(&self) -> &LineIndex {
        self.line_index.as_ref()
    }

    /// The column unit of the LSP client, to convert spans into LSP ranges.
    pub fn encoding_kind(&self) -> EncodingKind {
        self.encoding_kind
    }

    pub fn line_index_arc(&self) -> Arc<LineIndex> {
        Arc::clone(&self.line_index)
    }

    /// Get the parsed AST (always exists)
    pub fn ast(&self) -> Arc<tombi_ast_syntax::Root> {
        Arc::clone(&self.ast)
    }

    /// Get AST generation errors
    pub fn ast_errors(&self) -> &[tombi_diagnostic::Diagnostic] {
        &self.ast_errors
    }

    /// Get the parsed DocumentTree (always exists)
    pub fn document_tree(&self) -> Arc<tombi_document_tree_syntax::DocumentTree> {
        Arc::clone(&self.document_tree)
    }

    /// Get DocumentTree generation errors
    pub fn document_tree_errors(&self) -> &[tombi_diagnostic::Diagnostic] {
        &self.document_tree_errors
    }
}

fn build_document_tree(
    ast: &tombi_ast_syntax::Root,
    toml_version: tombi_config::TomlVersion,
) -> (
    Arc<tombi_document_tree_syntax::DocumentTree>,
    Vec<tombi_diagnostic::Diagnostic>,
) {
    let (document_tree, errors) = ast
        .clone()
        .into_document_tree_and_errors(toml_version)
        .into();
    let mut document_tree_errors = Vec::with_capacity(errors.len());
    for error in errors {
        error.set_diagnostics(&mut document_tree_errors);
    }
    (Arc::new(document_tree), document_tree_errors)
}

#[cfg(test)]
mod tests {
    use tombi_config::TomlVersion;
    use tombi_text::EncodingKind;

    use super::DocumentSource;

    #[test]
    fn line_index_arc_keeps_original_text_alive() {
        let mut document_source = DocumentSource::new(
            tombi_parser::parse("name = \"before\"\nversion = \"1.0.0\""),
            Some(1),
            TomlVersion::default(),
            EncodingKind::Utf16,
        );
        let line_index = document_source.line_index_arc();

        document_source.set_text("name = \"after\"", TomlVersion::default());

        assert_eq!(line_index.line_text(1), Some("version = \"1.0.0\""));
    }
}
