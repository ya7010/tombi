mod error;
mod key;
mod root;
pub mod support;
mod value;
mod value_type;

pub use error::Error;
pub use key::{Key, KeyKind};
pub use root::DocumentTree;
use root::RootItem;
use toml_version::TomlVersion;
pub use value::{
    Array, ArrayKind, Boolean, Float, Integer, IntegerKind, LocalDate, LocalDateTime, LocalTime,
    OffsetDateTime, String, StringKind, Table, TableKind, Value,
};
pub use value_type::ValueType;

/// A structure that holds an incomplete tree and errors that are the reason for the incompleteness.
///
/// [DocumentTree](crate::Root) needs to hold an incomplete tree and errors at the same time because it allows incomplete values.
/// If there are no errors, the tree is considered complete and can be converted to a [Document](document::Document).
pub struct DocumentTreeResult<T> {
    pub tree: T,
    pub errors: Vec<crate::Error>,
}

impl<T> DocumentTreeResult<T> {
    pub(crate) fn map<F>(self, f: impl FnOnce(T) -> F) -> DocumentTreeResult<F> {
        DocumentTreeResult {
            tree: f(self.tree),
            errors: self.errors,
        }
    }

    pub fn ok(self) -> Result<T, Vec<crate::Error>> {
        if self.errors.is_empty() {
            Ok(self.tree)
        } else {
            Err(self.errors)
        }
    }
}

impl<T> From<DocumentTreeResult<T>> for (T, Vec<crate::Error>) {
    fn from(result: DocumentTreeResult<T>) -> Self {
        (result.tree, result.errors)
    }
}

pub trait ValueImpl {
    fn value_type(&self) -> ValueType;

    fn range(&self) -> text::Range;
}

/// A structure that holds an incomplete tree and errors that are the reason for the incompleteness.
pub trait IntoDocumentTreeResult<T> {
    fn into_document_tree_result(self, toml_version: TomlVersion) -> DocumentTreeResult<T>;
}

/// Get a complete tree or errors for incomplete reasons.
pub trait TryIntoDocumentTree<T> {
    fn try_into_document_tree(self, toml_version: TomlVersion) -> Result<T, Vec<crate::Error>>;
}

impl<T, U> TryIntoDocumentTree<T> for U
where
    U: IntoDocumentTreeResult<T>,
{
    #[inline]
    fn try_into_document_tree(self, toml_version: TomlVersion) -> Result<T, Vec<crate::Error>> {
        self.into_document_tree_result(toml_version).ok()
    }
}
