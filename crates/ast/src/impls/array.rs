use crate::{support, AstNode};
use config::TomlVersion;
use itertools::Itertools;
use syntax::{SyntaxKind::*, T};

impl crate::Array {
    #[inline]
    pub fn inner_begin_dangling_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::node::begin_dangling_comments(
            self.syntax()
                .children_with_tokens()
                .skip_while(|node| node.kind() != T!('['))
                .skip(1),
        )
    }

    #[inline]
    pub fn inner_end_dangling_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::node::end_dangling_comments(
            self.syntax()
                .children_with_tokens()
                .take_while(|node| node.kind() != T!(']')),
        )
    }

    #[inline]
    pub fn dangling_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::node::dangling_comments(self.syntax().children_with_tokens())
    }

    #[inline]
    pub fn values_with_comma(&self) -> impl Iterator<Item = (crate::Value, Option<crate::Comma>)> {
        self.values()
            .zip_longest(support::node::children::<crate::Comma>(self.syntax()))
            .map(|value_with_comma| match value_with_comma {
                itertools::EitherOrBoth::Both(value, comma) => (value, Some(comma)),
                itertools::EitherOrBoth::Left(value) => (value, None),
                itertools::EitherOrBoth::Right(_) => unreachable!(),
            })
    }

    pub fn should_be_multiline(&self, toml_version: TomlVersion) -> bool {
        self.has_tailing_comma_after_last_value()
            || self.has_multiline_values(toml_version)
            || self.has_inner_comments()
    }

    pub fn has_tailing_comma_after_last_value(&self) -> bool {
        self.syntax()
            .children_with_tokens()
            .collect_vec()
            .into_iter()
            .rev()
            .skip_while(|item| item.kind() != T!(']'))
            .skip(1)
            .find(|item| !matches!(item.kind(), WHITESPACE | COMMENT | LINE_BREAK))
            .map_or(false, |it| it.kind() == T!(,))
    }

    pub fn has_multiline_values(&self, toml_version: TomlVersion) -> bool {
        self.values().any(|value| match value {
            crate::Value::Array(array) => array.should_be_multiline(toml_version),
            crate::Value::InlineTable(inline_table) => {
                inline_table.should_be_multiline(toml_version)
            }
            crate::Value::MultiLineBasicString(string) => {
                string.token().unwrap().text().contains('\n')
            }
            crate::Value::MultiLineLiteralString(string) => {
                string.token().unwrap().text().contains('\n')
            }
            _ => false,
        })
    }

    pub fn has_inner_comments(&self) -> bool {
        support::node::has_inner_comments(self.syntax().children_with_tokens(), T!('['), T!(']'))
    }
}
