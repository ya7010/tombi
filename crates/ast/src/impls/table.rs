use crate::{support, AstNode, TableOrArrayOfTable};
use syntax::T;

impl crate::Table {
    pub fn header_leading_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::leading_comments(self.syntax().children_with_tokens())
    }

    pub fn header_tailing_comment(&self) -> Option<crate::Comment> {
        support::tailing_comment(self.syntax().children_with_tokens(), T!(']'))
    }

    /// Returns an iterator over the subtables of this table.
    ///
    /// ```toml
    /// [foo]  # <- This is a self table
    /// [foo.bar]  # <- This is a subtable
    /// key = "value"
    ///
    /// [[foo.bar.baz]]  # <- This is also a subtable
    /// key = true
    /// ```
    pub fn subtables<'a>(&'a self) -> impl Iterator<Item = TableOrArrayOfTable> + 'a {
        support::next_siblings_nodes(self).take_while(|t: &TableOrArrayOfTable| {
            t.header()
                .unwrap()
                .to_string()
                .starts_with(&self.header().unwrap().to_string())
        })
    }

    pub fn parent_tables<'a>(&'a self) -> impl Iterator<Item = TableOrArrayOfTable> + 'a {
        support::prev_siblings_nodes(self).take_while(|t: &TableOrArrayOfTable| {
            self.header()
                .unwrap()
                .to_string()
                .starts_with(&t.header().unwrap().to_string())
        })
    }
}
