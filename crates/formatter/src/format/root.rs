use ast::AstNode;

use super::{
    comment::{BeginDanglingComment, DanglingComment, EndDanglingComment},
    Format,
};
use std::fmt::Write;

impl Format for ast::Root {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        f.reset();

        let items = self.items().collect::<Vec<_>>();
        if !items.is_empty() {
            self.begin_dangling_comments()
                .map(BeginDanglingComment)
                .collect::<Vec<_>>()
                .fmt(f)?;

            items
                .into_iter()
                .fold(
                    (Header::Root { item_size: 0 }, vec![]),
                    |(mut header, mut acc), item| match &item {
                        ast::RootItem::Table(table) => {
                            let header_text = table.header().unwrap().syntax().to_string();

                            match header {
                                Header::Root { item_size } => {
                                    if item_size > 0 {
                                        acc.push(ItemOrNewLine::NewLine);
                                    }
                                }
                                Header::Table {
                                    header_text: pre_header_text,
                                    item_size,
                                } => {
                                    if item_size > 0 || !header_text.starts_with(&pre_header_text) {
                                        acc.push(ItemOrNewLine::NewLine);
                                    }
                                }
                                Header::ArrayOfTable { .. } => {
                                    acc.push(ItemOrNewLine::NewLine);
                                }
                            };
                            acc.push(ItemOrNewLine::Item(item));

                            (
                                Header::Table {
                                    header_text,
                                    item_size: 0,
                                },
                                acc,
                            )
                        }
                        ast::RootItem::ArrayOfTable(_) => {
                            if !header.is_root() {
                                acc.push(ItemOrNewLine::NewLine);
                            }
                            acc.push(ItemOrNewLine::Item(item));

                            (Header::ArrayOfTable { item_size: 0 }, acc)
                        }
                        ast::RootItem::KeyValue(_) => {
                            acc.push(ItemOrNewLine::Item(item));
                            header.inc_item();

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

            self.end_dangling_comments()
                .map(EndDanglingComment)
                .collect::<Vec<_>>()
                .fmt(f)?;
        } else {
            self.dangling_comments()
                .map(DanglingComment)
                .collect::<Vec<_>>()
                .fmt(f)?;
        }

        Ok(())
    }
}

impl Format for ast::RootItem {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ast::RootItem::Table(it) => it.fmt(f),
            ast::RootItem::ArrayOfTable(it) => it.fmt(f),
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

enum Header {
    Root {
        item_size: usize,
    },

    Table {
        header_text: String,
        item_size: usize,
    },

    ArrayOfTable {
        item_size: usize,
    },
}

impl Header {
    #[inline]
    fn is_root(&self) -> bool {
        matches!(self, Self::Root { .. })
    }

    #[inline]
    fn inc_item(&mut self) {
        match self {
            Self::Root { item_size } => *item_size += 1,
            Self::Table { item_size, .. } => *item_size += 1,
            Self::ArrayOfTable { item_size } => *item_size += 1,
        }
    }
}
