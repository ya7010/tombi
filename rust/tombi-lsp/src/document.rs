#[derive(Debug)]
pub struct Document {
    ast: ast::Root,
    document: Option<document::Table>,
}

impl Document {
    pub fn new(ast: ast::Root) -> Self {
        Self {
            ast,
            document: None,
        }
    }

    pub fn ast(&self) -> &ast::Root {
        &self.ast
    }

    pub fn document(&mut self) -> &document::Table {
        // TODO: Implement this
        // if self.document.is_none() {
        //     self.document = Some(document::Table::new(&self.ast));
        // }
        self.document.as_ref().unwrap()
    }
}
