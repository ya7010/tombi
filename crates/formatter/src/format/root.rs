use ast::AstNode;

use super::{
    comment::{BeginDanglingComment, EndDanglingComment},
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
                .fold((None, vec![]), |(pre_header, mut acc), item| match &item {
                    ast::RootItem::Table(table) => {
                        let header_text = table.header().unwrap().syntax().to_string();
                        let item_size = table.key_values().count();

                        match pre_header {
                            Some(Header::Table {
                                header_text: pre_header_text,
                                item_size,
                            }) => {
                                if item_size > 0 || !header_text.starts_with(&pre_header_text) {
                                    acc.push(ItemOrNewLine::NewLine);
                                }
                            }
                            Some(Header::ArrayOfTable) => {
                                acc.push(ItemOrNewLine::NewLine);
                            }
                            None => {}
                        };

                        acc.push(ItemOrNewLine::Item(item));
                        (
                            Some(Header::Table {
                                header_text,
                                item_size,
                            }),
                            acc,
                        )
                    }
                    ast::RootItem::ArrayOfTable(_) => {
                        if pre_header.is_some() {
                            acc.push(ItemOrNewLine::NewLine);
                        }
                        acc.push(ItemOrNewLine::Item(item));
                        (Some(Header::ArrayOfTable), acc)
                    }
                    ast::RootItem::KeyValue(_) => {
                        acc.push(ItemOrNewLine::Item(item));
                        (None, acc)
                    }
                })
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
            self.dangling_comments().collect::<Vec<_>>().fmt(f)?;
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
    Table {
        header_text: String,
        item_size: usize,
    },
    ArrayOfTable,
}
