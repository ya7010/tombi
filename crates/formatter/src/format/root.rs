use super::{
    comment::{BeginDanglingComment, DanglingComment, EndDanglingComment},
    Format,
};
use itertools::Itertools;
use std::fmt::Write;

impl Format for ast::Root {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        f.reset();

        let items = self.items().collect_vec();
        if !items.is_empty() {
            self.begin_dangling_comments()
                .into_iter()
                .map(|comments| comments.into_iter().map(BeginDanglingComment).collect_vec())
                .collect_vec()
                .fmt(f)?;

            items
                .into_iter()
                .fold(
                    (Header::Root { key_value_size: 0 }, vec![]),
                    |(mut header, mut acc), item| match &item {
                        ast::RootItem::Table(table) => {
                            let header_keys = table.header().unwrap().keys();
                            let key_value_size = table.key_values().count();

                            match header {
                                Header::Root { key_value_size } => {
                                    if key_value_size > 0 {
                                        acc.push(ItemOrNewLine::NewLine);
                                    }
                                }
                                Header::Table {
                                    header_keys: pre_header_keys,
                                    key_value_size,
                                }
                                | Header::ArrayOfTables {
                                    header_keys: pre_header_keys,
                                    key_value_size,
                                } => {
                                    if key_value_size > 0
                                        || !header_keys.starts_with(&pre_header_keys)
                                    {
                                        acc.push(ItemOrNewLine::NewLine);
                                    }
                                }
                            };
                            acc.push(ItemOrNewLine::Item(item));

                            (
                                Header::Table {
                                    header_keys,
                                    key_value_size,
                                },
                                acc,
                            )
                        }
                        ast::RootItem::ArrayOfTables(array_of_tables) => {
                            let header_keys = array_of_tables.header().unwrap().keys();
                            let key_value_size = array_of_tables.key_values().count();

                            match header {
                                Header::Root { key_value_size } => {
                                    if key_value_size > 0 {
                                        acc.push(ItemOrNewLine::NewLine);
                                    }
                                }
                                Header::Table {
                                    header_keys: pre_header_keys,
                                    key_value_size,
                                } => {
                                    if key_value_size > 0
                                        || !header_keys.starts_with(&pre_header_keys)
                                    {
                                        acc.push(ItemOrNewLine::NewLine);
                                    }
                                }
                                Header::ArrayOfTables {
                                    header_keys: pre_header_keys,
                                    key_value_size,
                                } => {
                                    if key_value_size > 0
                                        || !header_keys.starts_with(&pre_header_keys)
                                        || pre_header_keys.same_as(&header_keys)
                                    {
                                        acc.push(ItemOrNewLine::NewLine);
                                    }
                                }
                            };
                            acc.push(ItemOrNewLine::Item(item));

                            (
                                Header::ArrayOfTables {
                                    header_keys,
                                    key_value_size,
                                },
                                acc,
                            )
                        }
                        ast::RootItem::KeyValue(_) => {
                            header = if let Header::Root { key_value_size } = header {
                                Header::Root {
                                    key_value_size: key_value_size + 1,
                                }
                            } else {
                                header
                            };
                            acc.push(ItemOrNewLine::Item(item));
                            (header, acc)
                        }
                    },
                )
                .1
                .into_iter()
                .enumerate()
                .try_for_each(|(i, item)| {
                    if i > 0 && matches!(item, ItemOrNewLine::Item(_)) {
                        ItemOrNewLine::NewLine.fmt(f)?;
                    }
                    item.fmt(f)
                })?;

            for comments in self.end_dangling_comments() {
                comments
                    .into_iter()
                    .map(EndDanglingComment)
                    .collect_vec()
                    .fmt(f)?;
            }
        } else {
            self.dangling_comments()
                .into_iter()
                .map(|comments| comments.into_iter().map(DanglingComment).collect_vec())
                .collect_vec()
                .fmt(f)?;
        }

        Ok(())
    }
}

impl Format for ast::RootItem {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ast::RootItem::Table(it) => it.fmt(f),
            ast::RootItem::ArrayOfTables(it) => it.fmt(f),
            ast::RootItem::KeyValue(it) => it.fmt(f),
        }
    }
}

enum ItemOrNewLine {
    Item(ast::RootItem),
    NewLine,
}

impl Format for ItemOrNewLine {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Item(it) => it.fmt(f),
            Self::NewLine => write!(f, "{}", f.line_ending()),
        }
    }
}

#[derive(Debug)]
enum Header {
    Root {
        key_value_size: usize,
    },

    Table {
        header_keys: ast::AstChildren<ast::Key>,
        key_value_size: usize,
    },

    ArrayOfTables {
        header_keys: ast::AstChildren<ast::Key>,
        key_value_size: usize,
    },
}

#[cfg(test)]
mod test {
    use crate::test_format;

    test_format! {
        #[test]
        fn empty_table_space_on_own_subtable(
            r#"
            [foo]
            [foo.bar]
            "#
        ) -> Ok(source);
    }

    test_format! {
        #[test]
        fn empty_table_space_on_other_table(
            r#"
            [foo]

            [bar.baz]
            "#
        ) -> Ok(source);
    }

    test_format! {
        #[test]
        fn empty_table_space_on_own_array_of_subtables(
            r#"
            [foo]
            [[foo.bar]]
            "#
        ) -> Ok(source);
    }

    test_format! {
        #[test]
        fn empty_table_space_on_other_array_of_tables(
            r#"
            [foo]

            [[bar.baz]]
            "#
        ) -> Ok(source);
    }

    test_format! {
        #[test]
        fn empty_array_of_tables_space_on_own_subtable(
            r#"
            [[foo]]
            [foo.bar]
            "#
        ) -> Ok(source);
    }

    test_format! {
        #[test]
        fn empty_array_of_tables_space_on_other_subtable(
            r#"
            [[foo]]

            [bar.baz]
            "#
        ) -> Ok(source);
    }

    test_format! {
        #[test]
        fn empty_array_of_tables_space_on_same_array_of_tables(
            r#"
            [[foo]]

            [[foo]]
            "#
        ) -> Ok(source);
    }

    test_format! {
        #[test]
        fn only_dangling_comment1(
            r#"
            # root dangling comment
            "#
        ) -> Ok(source);
    }

    test_format! {
        #[test]
        fn only_dangling_comment2(
            r#"
            # root dangling comment 1-1
            # root dangling comment 1-2

            # root dangling comment 2-1
            # root dangling comment 2-1
            # root dangling comment 2-3

            # root dangling comment 3-1
            "#
        ) -> Ok(source);
    }
}
