use std::ops::Deref;

use toml_version::TomlVersion;

use crate::{
    support::comment::try_new_comment, DocumentTreeAndErrors, IntoDocumentTreeAndErrors, Table,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentTree(pub(crate) Table);

impl From<DocumentTree> for Table {
    fn from(tree: DocumentTree) -> Self {
        tree.0
    }
}

impl Deref for DocumentTree {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) enum RootItem {
    Table(Table),
    ArrayOfTables(Table),
    KeyValue(Table),
}

impl IntoDocumentTreeAndErrors<crate::DocumentTree> for ast::Root {
    fn into_document_tree_and_errors(
        self,
        toml_version: TomlVersion,
    ) -> crate::DocumentTreeAndErrors<crate::DocumentTree> {
        let mut tree = crate::DocumentTree(crate::Table::new_root(&self));
        let mut errors = Vec::new();

        for comments in self.begin_dangling_comments() {
            for comment in comments {
                if let Err(error) = try_new_comment(comment.as_ref()) {
                    errors.push(error);
                }
            }
        }

        for item in self.items() {
            let (item, errs) = item.into_document_tree_and_errors(toml_version).into();

            if !errs.is_empty() {
                errors.extend(errs);
            }

            match item {
                RootItem::Table(table)
                | RootItem::ArrayOfTables(table)
                | RootItem::KeyValue(table) => {
                    if let Err(errs) = tree.0.merge(table) {
                        errors.extend(errs);
                    }
                }
            }
        }

        for comments in self.end_dangling_comments() {
            for comment in comments {
                if let Err(error) = try_new_comment(comment.as_ref()) {
                    errors.push(error);
                }
            }
        }

        DocumentTreeAndErrors { tree, errors }
    }
}

impl IntoDocumentTreeAndErrors<crate::RootItem> for ast::RootItem {
    fn into_document_tree_and_errors(
        self,
        toml_version: TomlVersion,
    ) -> crate::DocumentTreeAndErrors<crate::RootItem> {
        match self {
            ast::RootItem::Table(table) => table
                .into_document_tree_and_errors(toml_version)
                .map(crate::RootItem::Table),
            ast::RootItem::ArrayOfTables(array) => array
                .into_document_tree_and_errors(toml_version)
                .map(crate::RootItem::ArrayOfTables),
            ast::RootItem::KeyValue(key_value) => key_value
                .into_document_tree_and_errors(toml_version)
                .map(crate::RootItem::KeyValue),
        }
    }
}
