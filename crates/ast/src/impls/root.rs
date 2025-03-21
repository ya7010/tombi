use itertools::Itertools;
use syntax::SyntaxKind;

use crate::{support, AstNode};

impl crate::Root {
    /// Returns the schema URL from the first dangling comment or the first key-value.
    ///
    /// ```toml
    /// #:schema "https://example.com/schema.json"
    /// key = "value"
    /// ```
    pub fn file_schema_url(
        &self,
        source_path: Option<&std::path::Path>,
    ) -> Option<(Result<url::Url, String>, text::Range)> {
        if let Some(comments) = itertools::chain!(
            self.key_values_begin_dangling_comments()
                .into_iter()
                .next()
                .map(|comment| {
                    comment
                        .into_iter()
                        .map(|comment| crate::Comment::from(comment))
                        .collect_vec()
                }),
            self.key_values_dangling_comments()
                .into_iter()
                .next()
                .map(|comment| {
                    comment
                        .into_iter()
                        .map(|comment| crate::Comment::from(comment))
                        .collect_vec()
                }),
            self.items().into_iter().next().map(|item| {
                item.leading_comments()
                    .into_iter()
                    .map(|comment| crate::Comment::from(comment))
                    .collect_vec()
            }),
        )
        .find(|comments| !comments.is_empty())
        {
            for comment in comments {
                if let Some((schema_url, url_range)) = comment.schema_url(source_path.as_deref()) {
                    return Some((schema_url, url_range));
                }
            }
        }
        None
    }

    #[inline]
    pub fn key_values(&self) -> impl Iterator<Item = crate::KeyValue> {
        self.items().into_iter().filter_map(|item| match item {
            crate::RootItem::KeyValue(key_value) => Some(key_value),
            _ => None,
        })
    }

    #[inline]
    pub fn table_or_array_of_tables(&self) -> impl Iterator<Item = crate::TableOrArrayOfTable> {
        self.items().into_iter().filter_map(|item| match item {
            crate::RootItem::Table(table) => Some(crate::TableOrArrayOfTable::Table(table)),
            crate::RootItem::ArrayOfTable(array_of_table) => {
                Some(crate::TableOrArrayOfTable::ArrayOfTable(array_of_table))
            }
            _ => None,
        })
    }

    pub fn key_values_begin_dangling_comments(&self) -> Vec<Vec<crate::BeginDanglingComment>> {
        support::node::begin_dangling_comments(self.syntax().children_with_tokens())
    }

    pub fn key_values_end_dangling_comments(&self) -> Vec<Vec<crate::EndDanglingComment>> {
        support::node::end_dangling_comments(
            self.syntax()
                .children_with_tokens()
                .take_while(|node_or_token| node_or_token.kind() != SyntaxKind::TABLE),
        )
    }

    pub fn key_values_dangling_comments(&self) -> Vec<Vec<crate::DanglingComment>> {
        support::node::dangling_comments(self.syntax().children_with_tokens())
    }
}
