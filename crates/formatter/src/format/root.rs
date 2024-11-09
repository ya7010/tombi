use ast::AstNode;

use super::{
    comment::{BeginDanglingComment, EndDanglingComment},
    Format,
};
use std::fmt::Write;

impl Format for ast::Root {
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        f.reset_ident();

        self.begin_dangling_comments()
            .map(BeginDanglingComment)
            .collect::<Vec<_>>()
            .fmt(f)?;

        self.items()
            .fold((None, vec![]), |(pre_header, mut acc), item| match &item {
                ast::RootItem::Table(table) => {
                    let headers = table.header().unwrap().syntax().to_string();
                    match pre_header {
                        Some(Header::Table(pre_table_headers)) => {
                            if !headers.starts_with(&pre_table_headers) {
                                acc.push(ItemOrNewLine::NewLine);
                            }
                        }
                        Some(Header::ArrayOfTable) => {
                            acc.push(ItemOrNewLine::NewLine);
                        }
                        None => {}
                    };

                    acc.push(ItemOrNewLine::Item(item));
                    (Some(Header::Table(headers)), acc)
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
    Table(String),
    ArrayOfTable,
}
