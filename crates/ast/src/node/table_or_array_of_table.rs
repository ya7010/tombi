use crate::{ArrayOfTables, AstNode, Keys, Table};

#[derive(Debug, Clone)]
pub enum TableOrArrayOfTable {
    Table(Table),
    ArrayOfTables(ArrayOfTables),
}

impl TableOrArrayOfTable {
    pub fn header(&self) -> Option<Keys> {
        match self {
            TableOrArrayOfTable::Table(table) => table.header(),
            TableOrArrayOfTable::ArrayOfTables(array_of_tables) => array_of_tables.header(),
        }
    }

    pub fn range(&self) -> text::Range {
        match self {
            TableOrArrayOfTable::Table(table) => table.range(),
            TableOrArrayOfTable::ArrayOfTables(array_of_tables) => array_of_tables.range(),
        }
    }
}

impl AstNode for TableOrArrayOfTable {
    #[inline]
    fn can_cast(kind: syntax::SyntaxKind) -> bool {
        Table::can_cast(kind) || ArrayOfTables::can_cast(kind)
    }

    #[inline]
    fn cast(syntax: syntax::SyntaxNode) -> Option<Self> {
        if Table::can_cast(syntax.kind()) {
            Table::cast(syntax).map(TableOrArrayOfTable::Table)
        } else if ArrayOfTables::can_cast(syntax.kind()) {
            ArrayOfTables::cast(syntax).map(TableOrArrayOfTable::ArrayOfTables)
        } else {
            None
        }
    }

    #[inline]
    fn syntax(&self) -> &syntax::SyntaxNode {
        match self {
            TableOrArrayOfTable::Table(table) => table.syntax(),
            TableOrArrayOfTable::ArrayOfTables(array_of_tables) => array_of_tables.syntax(),
        }
    }
}
