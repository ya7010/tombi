use std::fmt::Write;

use itertools::Itertools;

use super::Format;

impl Format for ast::Root {
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        f.reset();

        let items = self.items().collect_vec();
        if !items.is_empty() {
            self.begin_dangling_comments().format(f)?;

            items
                .into_iter()
                .fold(
                    (Header::Root { key_value_size: 0 }, vec![]),
                    |(mut header, mut acc), item| match &item {
                        ast::RootItem::Table(table) => {
                            let header_keys = table.header().unwrap().keys();
                            let key_value_size = table.key_values().count();
                            let has_dangling_comments = !table.dangling_comments().is_empty();

                            match header {
                                Header::Root { key_value_size } => {
                                    if key_value_size > 0 {
                                        acc.push(ItemOrNewLine::NewLine);
                                    }
                                }
                                Header::Table {
                                    header_keys: pre_header_keys,
                                    key_value_size,
                                    has_dangling_comments,
                                }
                                | Header::ArrayOfTables {
                                    header_keys: pre_header_keys,
                                    key_value_size,
                                    has_dangling_comments,
                                } => {
                                    if key_value_size > 0
                                        || !header_keys.starts_with(&pre_header_keys)
                                        || has_dangling_comments
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
                                    has_dangling_comments,
                                },
                                acc,
                            )
                        }
                        ast::RootItem::ArrayOfTables(array_of_tables) => {
                            let header_keys = array_of_tables.header().unwrap().keys();
                            let key_value_size = array_of_tables.key_values().count();
                            let has_dangling_comments =
                                !array_of_tables.dangling_comments().is_empty();

                            match header {
                                Header::Root { key_value_size } => {
                                    if key_value_size > 0 {
                                        acc.push(ItemOrNewLine::NewLine);
                                    }
                                }
                                Header::Table {
                                    header_keys: pre_header_keys,
                                    key_value_size,
                                    has_dangling_comments,
                                } => {
                                    if key_value_size > 0
                                        || !header_keys.starts_with(&pre_header_keys)
                                        || has_dangling_comments
                                    {
                                        acc.push(ItemOrNewLine::NewLine);
                                    }
                                }
                                Header::ArrayOfTables {
                                    header_keys: pre_header_keys,
                                    key_value_size,
                                    has_dangling_comments,
                                } => {
                                    if key_value_size > 0
                                        || !header_keys.starts_with(&pre_header_keys)
                                        || pre_header_keys.same_as(&header_keys)
                                        || has_dangling_comments
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
                                    has_dangling_comments,
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
                        ItemOrNewLine::NewLine.format(f)?;
                    }
                    item.format(f)
                })?;

            self.end_dangling_comments().format(f)?;
        } else {
            self.dangling_comments().format(f)?;
        }

        Ok(())
    }
}

impl Format for ast::RootItem {
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ast::RootItem::Table(it) => it.format(f),
            ast::RootItem::ArrayOfTables(it) => it.format(f),
            ast::RootItem::KeyValue(it) => it.format(f),
        }
    }
}

enum ItemOrNewLine {
    Item(ast::RootItem),
    NewLine,
}

impl Format for ItemOrNewLine {
    fn format(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Item(it) => it.format(f),
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
        has_dangling_comments: bool,
    },

    ArrayOfTables {
        header_keys: ast::AstChildren<ast::Key>,
        key_value_size: usize,
        has_dangling_comments: bool,
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
        fn empty_table_space_on_other_array_of_tables_with_comments(
            r#"
            [foo]  # header table comment
            # table dangling comment 1-1
            # table dangling comment 1-2

            # table dangling comment 2-1
            # table dangling comment 2-2
            # table dangling comment 2-3

            # table dangling comment 3-1

            # table header leading comment1
            # table header leading comment2
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
        fn empty_array_of_tables_space_on_own_subtable_with_comments(
            r#"
            [[foo]]  # header tailing comment
            # table dangling comment 1-1
            # table dangling comment 1-2

            # table dangling comment 2-1
            # table dangling comment 2-2
            # table dangling comment 2-3

            # table dangling comment 3-1

            # table header leading comment1
            # table header leading comment2
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
        fn empty_array_of_tables_space_on_other_subtable_with_comments(
            r#"
            [[foo]]  # header tailing comment
            # table dangling comment 1-1
            # table dangling comment 1-2

            # table dangling comment 2-1
            # table dangling comment 2-2
            # table dangling comment 2-3

            # table dangling comment 3-1

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
        fn empty_array_of_tables_space_on_same_array_of_tables_with_comment(
            r#"
            [[foo]]  # header tailing comment
            # table dangling comment 1-1
            # table dangling comment 1-2

            # table dangling comment 2-1
            # table dangling comment 2-2
            # table dangling comment 2-3

            # table dangling comment 3-1

            # table header leading comment1
            # table header leading comment2
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
