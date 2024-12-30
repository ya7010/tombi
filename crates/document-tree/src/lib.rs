mod error;
mod key;
pub mod support;
mod value;

pub use error::Error;
pub use key::{Key, KeyKind};
use std::ops::Deref;
use support::comment::try_new_comment;
use toml_version::TomlVersion;
pub use value::{
    Array, ArrayKind, Boolean, Float, Integer, IntegerKind, LocalDate, LocalDateTime, LocalTime,
    OffsetDateTime, String, StringKind, Table, TableKind, Value,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Root(Table);

impl From<Root> for Table {
    fn from(root: Root) -> Self {
        root.0
    }
}

impl Deref for Root {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum RootItem {
    Table(Table),
    ArrayOfTables(Table),
    KeyValue(Table),
}

pub trait TryIntoDocumentTree<T> {
    fn try_into_document_tree(self, toml_version: TomlVersion) -> Result<T, Vec<crate::Error>>;
}

impl TryIntoDocumentTree<Root> for ast::Root {
    fn try_into_document_tree(self, toml_version: TomlVersion) -> Result<Root, Vec<crate::Error>> {
        let mut root = Root(Table::new_root(&self));
        let mut errors = Vec::new();

        for comments in self.begin_dangling_comments() {
            for comment in comments {
                if let Err(error) = try_new_comment(comment.as_ref()) {
                    errors.push(error);
                }
            }
        }

        for item in self.items() {
            if let Err(errs) = match item.try_into_document_tree(toml_version) {
                Ok(
                    RootItem::Table(table)
                    | RootItem::ArrayOfTables(table)
                    | RootItem::KeyValue(table),
                ) => root.0.merge(table),
                Err(errs) => Err(errs),
            } {
                errors.extend(errs);
            }
        }

        for comments in self.end_dangling_comments() {
            for comment in comments {
                if let Err(error) = try_new_comment(comment.as_ref()) {
                    errors.push(error);
                }
            }
        }

        if errors.is_empty() {
            Ok(root)
        } else {
            Err(errors)
        }
    }
}

impl TryIntoDocumentTree<RootItem> for ast::RootItem {
    fn try_into_document_tree(
        self,
        toml_version: TomlVersion,
    ) -> Result<RootItem, Vec<crate::Error>> {
        match self {
            ast::RootItem::Table(table) => table
                .try_into_document_tree(toml_version)
                .map(RootItem::Table),
            ast::RootItem::ArrayOfTables(array) => array
                .try_into_document_tree(toml_version)
                .map(RootItem::ArrayOfTables),
            ast::RootItem::KeyValue(key_value) => key_value
                .try_into_document_tree(toml_version)
                .map(RootItem::KeyValue),
        }
    }
}
