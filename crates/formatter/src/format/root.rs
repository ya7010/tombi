use ast::AstNode;

use super::Format;

impl Format for ast::Root {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        self.items()
            .fold((None, vec![]), |(pre_header, mut acc), item| match &item {
                ast::RootItem::Table(table) => {
                    let headers = table
                        .header()
                        .iter()
                        .map(|it| it.syntax().to_string())
                        .collect::<Vec<_>>()
                        .join(".");
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
            .map(|item| item.format(context))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Format for ast::RootItem {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        match self {
            Self::KeyValue(it) => it.format(context),
            Self::Table(it) => it.format(context),
            Self::ArrayOfTable(it) => it.format(context),
        }
    }
}

enum ItemOrNewLine {
    Item(ast::RootItem),
    NewLine,
}

impl Format for ItemOrNewLine {
    fn format<'a>(&self, context: &'a crate::Context<'a>) -> String {
        match self {
            Self::Item(it) => it.format(context),
            Self::NewLine => "".to_string(),
        }
    }
}

enum Header {
    Table(String),
    ArrayOfTable,
}
