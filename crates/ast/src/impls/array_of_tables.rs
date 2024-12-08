use crate::{support, AstChildren, AstNode, TableOrArrayOfTable};
use syntax::T;

impl crate::ArrayOfTables {
    pub fn header_leading_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::leading_comments(self.syntax().children_with_tokens())
    }

    pub fn header_tailing_comment(&self) -> Option<crate::Comment> {
        support::tailing_comment(self.syntax().children_with_tokens(), T!("]]"))
    }

    fn parent_tables<'a>(&'a self) -> impl Iterator<Item = TableOrArrayOfTable> + 'a {
        support::prev_siblings_nodes(self).take_while(|t: &TableOrArrayOfTable| {
            self.header()
                .unwrap()
                .keys()
                .starts_with(&t.header().unwrap().keys())
        })
    }

    pub fn array_of_tables_keys<'a>(
        &'a self,
    ) -> impl Iterator<Item = AstChildren<crate::Key>> + 'a {
        self.parent_tables()
            .filter_map(|parent_table| match parent_table {
                crate::TableOrArrayOfTable::ArrayOfTables(array_of_tables) => {
                    Some(array_of_tables.header().unwrap().keys())
                }
                _ => None,
            })
    }
}
