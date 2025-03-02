use ast::AstNode;
use schema_store::SchemaContext;

use crate::{change::Change, Edit};

pub struct Editor<'a> {
    root: ast::Root,
    #[allow(dead_code)]
    changes: Vec<Change>,
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

    pub async fn edit(self) -> ast::Root {
        let new_root = self.root.clone_for_update();
        let changes = new_root
            .edit(
                &[],
                self.schema_context
                    .root_schema
                    .and_then(|document_schema| document_schema.value_schema.as_ref()),
                self.schema_context
                    .root_schema
                    .map(|document_schema| &document_schema.schema_url),
                self.schema_context
                    .root_schema
                    .map(|document_schema| &document_schema.definitions),
                self.schema_context,
            )
            .await;

        for change in changes {
            match change {
                Change::ReplaceRange { old, new } => {
                    let start = old.start().index();
                    let end = old.end().index();
                    let parent = old.start().parent().unwrap();
                    parent.splice_children(start..end + 1, new);
                }
            }
        }

        new_root
    }
}
