use schema_store::SchemaContext;

use crate::ast_change::AstChange;
use crate::Edit;

pub struct Editor<'a> {
    root: ast::Root,
    #[allow(dead_code)]
    changes: Vec<AstChange>,
    schema_context: &'a SchemaContext<'a>,
}

impl<'a> Editor<'a> {
    pub fn new(root: ast::Root, schema_context: &'a SchemaContext<'a>) -> Self {
        Self {
            root,
            changes: vec![],
            schema_context,
        }
    }

    pub async fn edit(&mut self) {
        self.root
            .edit(
                &[],
                self.schema_context
                    .root_schema
                    .map(|document_schema| &document_schema.schema_url),
                self.schema_context
                    .root_schema
                    .and_then(|document_schema| document_schema.value_schema.as_ref()),
                self.schema_context
                    .root_schema
                    .map(|document_schema| &document_schema.definitions),
                self.schema_context,
            )
            .await;
    }
}
